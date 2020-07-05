use bytes::Bytes;
use std::fmt::{self, Display, Formatter};
use std::ops::{Bound, Deref, Index, RangeBounds};
use std::slice::SliceIndex;
use std::str::Utf8Error;

/// A slice into a reference-counted string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ArcStr(Bytes);

impl ArcStr {
    pub const fn new() -> Self {
        ArcStr(Bytes::new())
    }

    pub const fn from_static(text: &'static str) -> Self {
        ArcStr(Bytes::from_static(text.as_bytes()))
    }

    pub fn from_utf8(bytes: &[u8]) -> Result<Self, Utf8Error> {
        // use the standard library to make sure it's valid UTF-8
        let _ = std::str::from_utf8(bytes)?;

        // Safety: We just did the validity check.
        unsafe { Ok(ArcStr::from_utf8_unchecked(bytes)) }
    }

    /// Create a new [`ArcStr`] from raw bytes *without* checking that the
    /// string contains valid UTF-8.
    pub unsafe fn from_utf8_unchecked(bytes: &[u8]) -> Self {
        ArcStr(Bytes::copy_from_slice(bytes))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn copy_from_slice(data: &str) -> Self {
        ArcStr(Bytes::copy_from_slice(data.as_bytes()))
    }

    /// Return a slice of `self` for the provided range.
    ///
    /// This will increment the reference count for the underlying memory and
    /// return a new [`ArcStr`] handle pointing at the slice.
    pub fn slice<R>(&self, range: R) -> Option<Self>
    where
        R: RangeBounds<usize>,
    {
        let len = self.len();

        let begin = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&n) => n + 1,
            Bound::Excluded(&n) => n,
            Bound::Unbounded => len,
        };

        if !self.as_str().is_char_boundary(begin) || !self.as_str().is_char_boundary(end) {
            return None;
        }

        // Safety: We've just done our bounds checks.
        Some(ArcStr(self.0.slice(range)))
    }

    pub fn as_str(&self) -> &str {
        &*self
    }
}

impl Deref for ArcStr {
    type Target = str;

    fn deref(&self) -> &str {
        // Safety: You can only create an ArcStr from either a valid UTF-8
        // string or via the ArcStr::get() method which does the correct bounds
        // checks.
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl AsRef<str> for ArcStr {
    fn as_ref(&self) -> &str {
        &*self
    }
}

impl<S> PartialEq<S> for ArcStr
where
    str: PartialEq<S>,
{
    fn eq(&self, other: &S) -> bool {
        self.as_ref().eq(other)
    }
}

impl<'a> PartialEq<ArcStr> for &'a str {
    fn eq(&self, other: &ArcStr) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<str> for ArcStr {
    fn eq(&self, other: &str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<I> Index<I> for ArcStr
where
    I: SliceIndex<str>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        match self.as_ref().get(index) {
            Some(s) => s,
            None => panic!("Attempted to slice out of bounds"),
        }
    }
}

impl Display for ArcStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_impl_works() {
        let s = ArcStr::from_static("Hello, World!");

        let got: &str = &*s;

        assert_eq!(got, "Hello, World!");
    }

    #[test]
    fn index_into_the_str_slice() {
        let s = ArcStr::from_static("Hello, World!");

        let got = &s[..5];

        assert_eq!(got, "Hello");
    }

    #[test]
    fn create_a_sub_slice_into_the_string() {
        let text = ArcStr::from_static("💩🦀");
        let length = '💩'.len_utf8();

        // try indices immediately before/after a boundary
        assert_eq!(text.slice(..length).unwrap().as_str(), "💩");
        assert!(text.slice(..=length).is_none());
        assert!(text.slice(..1).is_none());

        // Note the deliberate out-of-bounds
        let max_index = text.as_str().len() + 1;

        // try every possible index and every possible range combination
        for start in 0..=max_index {
            for end in start..=max_index {
                assert_range_equal(&text, text.as_str(), start..);
                assert_range_equal(&text, text.as_str(), start..end);
                assert_range_equal(&text, text.as_str(), start..=end);
                assert_range_equal(&text, text.as_str(), ..=end);
                assert_range_equal(&text, text.as_str(), ..end);
            }
        }
    }

    fn assert_range_equal<R>(left: &ArcStr, right: &str, range: R)
    where
        R: RangeBounds<usize> + SliceIndex<str, Output = str> + Clone,
    {
        let got = left.slice(range.clone());
        let should_be = right.get(range);

        assert_eq!(got.as_deref(), should_be);

        // make sure this is valid UTF-8
        if let Some(ArcStr(bytes)) = got {
            assert!(std::str::from_utf8(&bytes).is_ok());
        }
    }
}
