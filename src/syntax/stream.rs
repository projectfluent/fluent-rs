use std::iter::Fuse;

#[derive(Clone, Debug)]
pub struct ParserStream<I>
    where I: Iterator
{
    iter: Fuse<I>,
    pub buf: Vec<char>,
    peek_index: usize,
    index: usize,

    pub ch: Option<char>,

    iter_end: bool,
    peek_end: bool,
}

impl<I: Iterator<Item = char>> ParserStream<I> {
    pub fn new(iterable: I) -> ParserStream<I> {
        let mut iter = iterable.into_iter().fuse();
        let ch = iter.next();

        ParserStream {
            iter: iter,
            buf: vec![],
            peek_index: 0,
            index: 0,
            ch: ch,

            iter_end: false,
            peek_end: false,
        }
    }

    pub fn current(&mut self) -> Option<char> {
        self.ch
    }

    pub fn current_is(&mut self, ch: char) -> bool {
        self.ch == Some(ch)
    }

    pub fn current_peek(&self) -> Option<char> {
        if self.peek_end {
            return None;
        }

        let diff = self.peek_index - self.index;

        if diff == 0 {
            self.ch
        } else {
            Some(self.buf[diff - 1])
        }
    }

    pub fn current_peek_is(&mut self, ch: char) -> bool {
        match self.current_peek() {
            Some(c) => ch == c,
            None => false,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        if self.peek_end {
            return None;
        }

        self.peek_index += 1;

        let diff = self.peek_index - self.index;

        if diff > self.buf.len() {
            match self.iter.next() {
                Some(c) => {
                    self.buf.push(c);
                }
                None => {
                    self.peek_end = true;
                    return None;
                }
            }
        }

        return Some(self.buf[diff - 1]);
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_peek_index(&self) -> usize {
        self.peek_index
    }

    pub fn peek_char_is(&mut self, c: char) -> bool {
        if self.peek_end {
            return false;
        }
        let ret = match self.peek() {
            Some(ch) if ch == c => true,
            _ => false,
        };

        self.peek_index -= 1;
        return ret;
    }

    pub fn reset_peek(&mut self) {
        self.peek_index = self.index;
        self.peek_end = self.iter_end;
    }

    pub fn skip_to_peek(&mut self) {
        let diff = self.peek_index - self.index;

        for _ in 0..diff {
            self.ch = Some(self.buf.remove(0));
        }

        self.index = self.peek_index;
    }
}

impl<I> Iterator for ParserStream<I>
    where I: Iterator<Item = char>
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.iter_end {
            return None;
        }

        self.ch = if self.buf.is_empty() {
            self.iter.next()
        } else {
            Some(self.buf.remove(0))
        };

        self.index += 1;

        if self.ch.is_none() {
            self.iter_end = true;
            self.peek_end = true;
        }

        self.peek_index = self.index;

        self.ch
    }
}
