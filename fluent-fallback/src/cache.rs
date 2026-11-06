use std::{
    cell::{RefCell, UnsafeCell},
    cmp::Ordering,
    pin::Pin,
    task::Context,
    task::Poll,
    task::Waker,
};

use crate::generator::{BundleIterator, BundleStream};
use futures::{ready, Stream};
use pin_cell::{PinCell, PinMut};

/// Backing store for the cached bundles.
///
/// Each element lives behind its own `Box`, so references handed out by
/// [`Cache::get`] and [`Cache::push_get`] stay valid no matter how the
/// `Vec` of boxes grows. This keeps the pin-safety guarantee of the
/// `chunky-vec` crate previously used here, without its fixed 32-element
/// chunks: a `Localization` usually caches one or two bundles, and the
/// first chunk alone used to reserve several kilobytes that were never
/// used (see issue #407).
type ChunkedItems<T> = Vec<Box<T>>;

pub struct Cache<I, R>
where
    I: Iterator,
{
    iter: RefCell<I>,
    items: UnsafeCell<ChunkedItems<I::Item>>,
    res: std::marker::PhantomData<R>,
}

impl<I, R> Cache<I, R>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: RefCell::new(iter),
            items: Default::default(),
            res: std::marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            let items = self.items.get();
            (*items).len()
        }
    }

    pub fn get(&self, index: usize) -> Option<&I::Item> {
        unsafe {
            let items = self.items.get();
            (&*items).get(index).map(|item| &**item)
        }
    }

    /// Push, immediately getting a reference to the element
    pub fn push_get(&self, new_value: I::Item) -> &I::Item {
        unsafe {
            let items = self.items.get();
            let items = &mut *items;
            items.push(Box::new(new_value));
            items.last().expect("an item was just pushed")
        }
    }
}

impl<I, R> Cache<I, R>
where
    I: BundleIterator + Iterator,
{
    pub fn prefetch(&self) {
        self.iter.borrow_mut().prefetch_sync();
    }
}

pub struct CacheIter<'a, I, R>
where
    I: Iterator,
{
    cache: &'a Cache<I, R>,
    curr: usize,
}

impl<'a, I, R> Iterator for CacheIter<'a, I, R>
where
    I: Iterator,
{
    type Item = &'a I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let cache_len = self.cache.len();
        match self.curr.cmp(&cache_len) {
            Ordering::Less => {
                // Cached value
                self.curr += 1;
                self.cache.get(self.curr - 1)
            }
            Ordering::Equal => {
                // Get the next item from the iterator
                let item = self.cache.iter.borrow_mut().next();
                self.curr += 1;
                if let Some(item) = item {
                    Some(self.cache.push_get(item))
                } else {
                    None
                }
            }
            Ordering::Greater => {
                // Ran off the end of the cache
                None
            }
        }
    }
}

impl<'a, I, R> IntoIterator for &'a Cache<I, R>
where
    I: Iterator,
{
    type Item = &'a I::Item;
    type IntoIter = CacheIter<'a, I, R>;

    fn into_iter(self) -> Self::IntoIter {
        CacheIter {
            cache: self,
            curr: 0,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct AsyncCache<S, R>
where
    S: Stream,
{
    stream: PinCell<S>,
    items: UnsafeCell<ChunkedItems<S::Item>>,
    // TODO: Should probably be an SmallVec<[Waker; 1]> or something? I guess
    // multiple pending wakes are not really all that common.
    pending_wakes: RefCell<Vec<Waker>>,
    res: std::marker::PhantomData<R>,
}

impl<S, R> AsyncCache<S, R>
where
    S: Stream,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream: PinCell::new(stream),
            items: Default::default(),
            pending_wakes: Default::default(),
            res: std::marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            let items = self.items.get();
            (*items).len()
        }
    }

    pub fn get(&self, index: usize) -> Poll<Option<&S::Item>> {
        unsafe {
            let items = self.items.get();
            (&*items).get(index).map(|item| &**item).into()
        }
    }

    /// Push, immediately getting a reference to the element
    pub fn push_get(&self, new_value: S::Item) -> &S::Item {
        unsafe {
            let items = self.items.get();
            let items = &mut *items;
            items.push(Box::new(new_value));
            items.last().expect("an item was just pushed")
        }
    }

    pub fn stream(&self) -> AsyncCacheStream<'_, S, R> {
        AsyncCacheStream {
            cache: self,
            curr: 0,
        }
    }
}

impl<S, R> AsyncCache<S, R>
where
    S: BundleStream + Stream,
{
    pub async fn prefetch(&self) {
        let pin = unsafe { Pin::new_unchecked(&self.stream) };
        unsafe { PinMut::as_mut(&mut pin.borrow_mut()).get_unchecked_mut() }
            .prefetch_async()
            .await;
    }
}

impl<S, R> AsyncCache<S, R>
where
    S: Stream,
{
    // Helper function that gets the next value from wrapped stream.
    fn poll_next_item(&self, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        let pin = unsafe { Pin::new_unchecked(&self.stream) };
        let poll = PinMut::as_mut(&mut pin.borrow_mut()).poll_next(cx);
        if poll.is_ready() {
            let wakers = std::mem::take(&mut *self.pending_wakes.borrow_mut());
            for waker in wakers {
                waker.wake();
            }
        } else {
            self.pending_wakes.borrow_mut().push(cx.waker().clone());
        }
        poll
    }
}

pub struct AsyncCacheStream<'a, S, R>
where
    S: Stream,
{
    cache: &'a AsyncCache<S, R>,
    curr: usize,
}

impl<'a, S, R> Stream for AsyncCacheStream<'a, S, R>
where
    S: Stream,
{
    type Item = &'a S::Item;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let cache_len = self.cache.len();
        match self.curr.cmp(&cache_len) {
            Ordering::Less => {
                // Cached value
                self.curr += 1;
                self.cache.get(self.curr - 1)
            }
            Ordering::Equal => {
                // Get the next item from the stream
                let item = ready!(self.cache.poll_next_item(cx));
                self.curr += 1;
                if let Some(item) = item {
                    Some(self.cache.push_get(item)).into()
                } else {
                    None.into()
                }
            }
            Ordering::Greater => {
                // Ran off the end of the cache
                None.into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_yields_all_items() {
        let cache: Cache<std::ops::Range<usize>, ()> = Cache::new(0..3);
        let items: Vec<usize> = cache.into_iter().copied().collect();
        assert_eq!(items, vec![0, 1, 2]);
        assert_eq!(cache.len(), 3);
    }

    /// The cache hands out `&T` which callers may hold while the cache
    /// keeps growing. Boxing every item keeps those references valid,
    /// mirroring the pin-safety guarantee of `ChunkyVec`.
    #[test]
    fn references_stay_stable_across_growth() {
        let cache: Cache<std::ops::Range<usize>, ()> = Cache::new(0..5);
        let mut iter = cache.into_iter();

        let first: *const usize = iter.next().unwrap();
        let second: *const usize = iter.next().unwrap();

        // Grow the backing storage well past its previous capacity.
        for _ in 0..3 {
            iter.next().unwrap();
        }

        assert_eq!(unsafe { *first }, 0);
        assert_eq!(unsafe { *second }, 1);
        assert_eq!(cache.get(1), Some(&1));
        assert_eq!(cache.len(), 5);
    }

    /// A bundle-sized element (~200 bytes, see issue #407) must not
    /// trigger a large up-front allocation: `ChunkyVec` reserved
    /// 32 elements (32 x 200 = 6400 bytes) on the first push, while a
    /// `Localization` usually caches only one or two bundles.
    ///
    /// The assertion counts allocations in a narrow band around the old
    /// chunk size so unrelated test-harness allocations cannot flake it.
    #[test]
    fn caching_few_bundles_avoids_chunk_sized_allocations() {
        use std::alloc::{GlobalAlloc, Layout, System};
        use std::sync::atomic::{AtomicUsize, Ordering};

        #[repr(C)]
        struct BundleSized([u8; 200]);

        const CHUNK_BYTES_MIN: usize = 6000;
        const CHUNK_BYTES_MAX: usize = 7000;

        struct Counting;
        static IN_BAND: AtomicUsize = AtomicUsize::new(0);

        unsafe impl GlobalAlloc for Counting {
            unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
                let ptr = System.alloc(layout);
                if !ptr.is_null()
                    && layout.size() >= CHUNK_BYTES_MIN
                    && layout.size() < CHUNK_BYTES_MAX
                {
                    IN_BAND.fetch_add(1, Ordering::Relaxed);
                }
                ptr
            }

            unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
                System.dealloc(ptr, layout)
            }
        }

        #[global_allocator]
        static ALLOCATOR: Counting = Counting;

        IN_BAND.store(0, Ordering::Relaxed);

        let bundles = (0..2).map(|i| BundleSized([i as u8; 200]));
        let cache: Cache<_, ()> = Cache::new(bundles);
        let count = cache.into_iter().count();
        assert_eq!(count, 2);

        assert_eq!(
            IN_BAND.load(Ordering::Relaxed),
            0,
            "caching two ~200 byte bundles requested a chunk-sized allocation"
        );
    }
}
