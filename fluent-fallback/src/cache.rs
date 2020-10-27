use std::{
    cell::{RefCell, UnsafeCell},
    cmp::Ordering,
    iter,
    pin::Pin,
    task::Context,
    task::Poll,
};

use chunky_vec::ChunkyVec;
use futures::{ready, stream, Stream, StreamExt};
use pin_cell::{PinCell, PinMut};

pub struct Cache<I>
where
    I: Iterator,
{
    iter: RefCell<iter::Fuse<I>>,
    items: UnsafeCell<ChunkyVec<I::Item>>,
}

impl<I> Cache<I>
where
    I: Iterator,
{
    pub fn new<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = I::Item, IntoIter = I>,
    {
        Self {
            iter: RefCell::new(iter.into_iter().fuse()),
            items: Default::default(),
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
            (*items).get(index)
        }
    }

    /// Push, immediately getting a reference to the element
    pub fn push_get(&self, new_value: I::Item) -> &I::Item {
        unsafe {
            let items = self.items.get();
            (*items).push_get(new_value)
        }
    }
}

pub struct CacheIter<'a, I>
where
    I: Iterator,
{
    cache: &'a Cache<I>,
    curr: usize,
}

impl<'a, I> Iterator for CacheIter<'a, I>
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

impl<'a, I> IntoIterator for &'a Cache<I>
where
    I: Iterator,
{
    type Item = &'a I::Item;
    type IntoIter = CacheIter<'a, I>;

    fn into_iter(self) -> Self::IntoIter {
        CacheIter {
            cache: self,
            curr: 0,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct AsyncCache<S>
where
    S: Stream,
{
    stream: PinCell<stream::Fuse<S>>,
    items: UnsafeCell<ChunkyVec<S::Item>>,
}

impl<S> AsyncCache<S>
where
    S: Stream,
{
    pub fn new(stream: S) -> Self {
        AsyncCache {
            stream: PinCell::new(stream.fuse()),
            items: Default::default(),
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
            (*items).get(index).into()
        }
    }

    /// Push, immediately getting a reference to the element
    pub fn push_get(&self, new_value: S::Item) -> &S::Item {
        unsafe {
            let items = self.items.get();
            (*items).push_get(new_value)
        }
    }

    pub fn stream(&self) -> AsyncCacheStream<'_, S> {
        AsyncCacheStream {
            cache: self,
            curr: 0,
        }
    }
}

impl<S> AsyncCache<S>
where
    S: Stream,
{
    // Helper function that gets the next value from wrapped stream.
    fn poll_next_item(&self, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        let pin = unsafe { Pin::new_unchecked(&self.stream) };
        PinMut::as_mut(&mut pin.borrow_mut()).poll_next(cx)
    }
}

pub struct AsyncCacheStream<'a, S>
where
    S: Stream,
{
    cache: &'a AsyncCache<S>,
    curr: usize,
}

impl<'a, S> Stream for AsyncCacheStream<'a, S>
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
