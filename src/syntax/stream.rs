use super::errors::ParserError;

use std::iter::Fuse;

use std::result;

type Result<T> = result::Result<T, ParserError>;

#[derive(Clone, Debug)]
pub struct ParserStream<I>
    where I: Iterator
{
    iter: Fuse<I>,
    buf: Vec<char>,
    peek_index: Option<usize>,
    index: Option<usize>,

    ch: Option<char>,
}

impl<I: Iterator<Item = char>> ParserStream<I> {
    pub fn current(&mut self) -> Option<char> {
        self.ch
    }

    pub fn current_peek(&self) -> Option<&char> {
        match self.peek_index {
            Some(i) if i < self.buf.len() => Some(&self.buf[i]),
            _ => None,
        }
    }

    pub fn bump(&mut self) {
        self.peek_index = None;
        if self.buf.is_empty() {
            self.iter.next();
        } else {
            self.buf.remove(0);
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        match self.peek_index {
            Some(i) if i < self.buf.len() - 1 => {
                let ret = Some(&self.buf[i]);
                self.peek_index = Some(i + 1);
                return ret;
            }
            _ => {
                match self.iter.next() {
                    Some(x) => {
                        self.buf.push(x);
                        let i = self.buf.len() - 1;
                        self.peek_index = Some(i);
                        let ret = Some(&self.buf[i]);
                        return ret;
                    }
                    None => {
                        self.peek_index = None;
                        return None;
                    }
                }
            }
        }
    }

    pub fn get_index(&self) -> Option<usize> {
        self.index
    }

    pub fn get_peek_index(&self) -> Option<usize> {
        self.peek_index
    }

    pub fn reset_peek(&mut self) {
        self.peek_index = None;
    }

    pub fn skip_to_peek(&mut self) {
        self.buf.clear();
        self.peek_index = None;
    }

    pub fn peek_line_ws(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch != ' ' && ch != '\t' {
                break;
            }
        }
    }

    pub fn skip_ws_lines(&mut self) {
        loop {
            self.peek_line_ws();

            println!("{:?}", self.current_peek());

            match self.current_peek() {
                Some(&'\n') => {
                    self.skip_to_peek();
                }
                _ => {
                    self.reset_peek();
                    break;
                }
            }
        }
    }

    pub fn skip_line_ws(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch != ' ' && ch != '\t' {
                self.reset_peek();
                break;
            }

            self.next();
        }
    }

    pub fn skip_ws(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch != ' ' && ch != '\n' && ch != '\t' && ch != '\r' {
                self.reset_peek();
                break;
            }

            self.next();
        }
    }

    pub fn expect_char(&mut self, ch: char) -> Result<()> {
        match self.next() {
            Some(ch2) if ch == ch2 => Ok(()),
            _ => Err(ParserError::ExpectedToken { token: ch }),
        }
    }

    pub fn take_char_if(&mut self, ch: char) -> bool {
        match self.peek() {
            Some(&ch2) if ch == ch2 => {
                self.next();
                true
            }
            _ => {
                self.reset_peek();
                false
            }
        }
    }

    pub fn take_char_after_line_ws_if(&mut self, ch2: char) -> bool {
        let mut i = 0;

        while let Some(&ch) = self.peek() {
            if ch != ' ' && ch != '\t' {
                if ch == ch2 {
                    i += 1;
                    for _ in 0..i {
                        self.next();
                    }
                    return true;
                } else {
                    self.reset_peek();
                    return false;
                }
            }

            i += 1;
        }

        self.reset_peek();
        return false;
    }

    pub fn take_char<F>(&mut self, f: F) -> Option<char>
        where F: Fn(char) -> bool
    {

        match self.peek() {
            Some(&ch) if f(ch) => {
                self.next();
                Some(ch)
            }
            _ => {
                self.reset_peek();
                None
            }
        }
    }

    pub fn peek_char_matches<F>(&mut self, f: F) -> bool
        where F: Fn(char) -> bool
    {

        match self.peek() {
            Some(&ch) if f(ch) => {
                self.reset_peek();
                true
            }
            _ => {
                self.reset_peek();
                false
            }
        }
    }

    pub fn is_id_start(&mut self) -> bool {
        let closure = |x| match x {
            'a'...'z' | 'A'...'Z' | '_' => true,
            _ => false,
        };

        return self.peek_char_matches(closure);
    }

    pub fn take_id_start(&mut self) -> Option<char> {
        let closure = |x| match x {
            'a'...'z' | 'A'...'Z' | '_' => true,
            _ => false,
        };

        match self.take_char(closure) {
            Some(ch) => Some(ch),
            None => None,
        }
    }

    pub fn take_id_char(&mut self) -> Option<char> {
        let closure = |x| match x {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' | '-' => true,
            _ => false,
        };

        match self.take_char(closure) {
            Some(ch) => Some(ch),
            None => None,
        }
    }

    pub fn take_kw_char(&mut self) -> Option<char> {
        let closure = |x| match x {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' | '-' | ' ' => true,
            _ => false,
        };

        match self.take_char(closure) {
            Some(ch) => Some(ch),
            None => None,
        }
    }
}

impl<I> Iterator for ParserStream<I>
    where I: Iterator<Item = char>
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.peek_index = None;
        if self.buf.is_empty() {
            self.ch = self.iter.next()
        } else {
            self.ch = Some(self.buf.remove(0))
        }
        if self.ch.is_none() {
            self.index = None;
        } else {
            match self.index {
                Some(i) => self.index = Some(i + 1),
                None => self.index = Some(0),
            }
        }
        self.ch
    }
}

pub fn parserstream<I>(iterable: I) -> ParserStream<I::IntoIter>
    where I: IntoIterator
{
    ParserStream {
        iter: iterable.into_iter().fuse(),
        buf: vec![],
        peek_index: None,
        index: None,
        ch: None,
    }
}
