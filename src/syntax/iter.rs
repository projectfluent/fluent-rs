use std::iter::Fuse;

#[derive(Clone, Debug)]
pub struct ParserStream<I>
    where I: Iterator
{
    iter: Fuse<I>,
    pub buf: Vec<char>,
    peek_index: i32,
    index: i32,

    pub ch: Option<char>,

    iter_end: bool,
    peek_end: bool,
}

impl<I: Iterator<Item = char>> ParserStream<I> {
    pub fn current(&mut self) -> Option<char> {
        self.ch
    }

    pub fn current_is(&mut self, ch: char) -> bool {
        self.ch == Some(ch)
    }

    pub fn current_peek(&self) -> Option<char> {
        let diff = self.peek_index - self.index;

        if diff == 0 {
            return self.ch;
        }

        if self.peek_end {
            return None;
        }

        return Some(self.buf[(diff - 1) as usize]);
    }

    pub fn current_peek_is(&mut self, ch: char) -> bool {
        match self.current_peek() {
            Some(c) => ch == c,
            None => false,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        if !self.peek_end {
            self.peek_index += 1;
        }
        match self.iter.next() {
            Some(c) => {
                self.buf.push(c);
                let diff = (self.peek_index - self.index) as usize;
                return Some(self.buf[diff - 1]);
            }
            None => {
                self.peek_end = true;
                return None;
            }
        }
    }

    pub fn get_index(&self) -> i32 {
        self.index
    }

    pub fn get_peek_index(&self) -> i32 {
        return self.peek_index;
    }

    pub fn has_more(&mut self) -> bool {
        let ret = self.peek().is_some();

        self.reset_peek();

        ret
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

        if self.buf.is_empty() {
            self.ch = self.iter.next();
        } else {
            self.ch = Some(self.buf.remove(0));
        }

        if !self.iter_end {
            self.index += 1;
        }

        if self.ch.is_none() {
            self.iter_end = true;
            self.peek_end = true;
        }

        self.peek_index = self.index;

        self.ch
    }
}

pub fn parserstream<I>(iterable: I) -> ParserStream<I::IntoIter>
    where I: IntoIterator
{
    ParserStream {
        iter: iterable.into_iter().fuse(),
        buf: vec![],
        peek_index: -1,
        index: -1,
        ch: None,

        iter_end: false,
        peek_end: false,
    }
}
