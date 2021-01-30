use std::ops::Range;
pub trait Slice<'s>: AsRef<str> + Clone + PartialEq {
    fn slice(&self, range: Range<usize>) -> Self;
    fn trim(&mut self);
    fn get(&self, idx: usize) -> Option<&u8>;
    fn bytes(&self) -> &[u8];
}

impl<'s> Slice<'s> for String {
    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        self[range].to_string()
    }

    #[inline]
    fn trim(&mut self) {
        *self = self.trim_end().to_string();
    }

    #[inline]
    fn get(&self, idx: usize) -> Option<&u8> {
        self.as_bytes().get(idx)
    }

    #[inline]
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'s> Slice<'s> for &'s str {
    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        &self[range]
    }

    #[inline]
    fn trim(&mut self) {
        *self = self.trim_end();
    }

    #[inline]
    fn get(&self, idx: usize) -> Option<&u8> {
        self.as_bytes().get(idx)
    }

    #[inline]
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}
