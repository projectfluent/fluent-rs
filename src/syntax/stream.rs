use super::errors::ParserError;

use std::iter::Fuse;

use std::result;

type Result<T> = result::Result<T, ParserError>;

#[derive(Clone, Debug)]
pub struct ParserStream<I>
    where I: Iterator
{
    iter: Fuse<I>,
    pub buf: Vec<char>,
    peek_index: i32,
    index: i32,

    ch: Option<char>,

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

    // pub fn bump(&mut self) {
    //     self.peek_index = None;
    //     if self.buf.is_empty() {
    //         self.iter.next();
    //     } else {
    //         self.buf.remove(0);
    //     }
    // }

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

    pub fn peek_line_ws(&mut self) {
        while let Some(ch) = self.peek() {
            if ch != ' ' && ch != '\t' {
                break;
            }
        }
    }

    pub fn skip_ws_lines(&mut self) {
        loop {
            self.peek_line_ws();

            match self.current_peek() {
                Some('\n') => {
                    self.skip_to_peek();
                    self.next();
                }
                _ => {
                    self.reset_peek();
                    break;
                }
            }
        }
    }

    pub fn skip_line_ws(&mut self) {
        while let Some(ch) = self.ch {
            if ch != ' ' && ch != '\t' {
                break;
            }

            self.next();
        }
    }

    pub fn skip_ws(&mut self) {
        while let Some(ch) = self.ch {
            if ch != ' ' && ch != '\n' && ch != '\t' && ch != '\r' {
                break;
            }

            self.next();
        }
    }

    pub fn expect_char(&mut self, ch: char) -> Result<()> {
        match self.ch {
            Some(ch2) if ch == ch2 => {
                self.next();
                Ok(())
            }
            _ => Err(ParserError::ExpectedToken { token: ch }),
        }
    }

    pub fn take_char_if(&mut self, ch: char) -> bool {
        match self.ch {
            Some(ch2) if ch == ch2 => {
                self.next();
                true
            }
            _ => false,
        }
    }

    pub fn take_char_after_line_ws_if(&mut self, ch2: char) -> bool {
        while let Some(ch) = self.current_peek() {
            if ch == ' ' || ch == '\t' {
                self.peek();
            } else {
                if ch == ch2 {
                    self.skip_to_peek();
                    return true;
                } else {
                    self.reset_peek();
                    return false;
                }
            }
        }

        return false;
    }

    pub fn take_char<F>(&mut self, f: F) -> Option<char>
        where F: Fn(char) -> bool
    {

        match self.ch {
            Some(ch) if f(ch) => {
                self.next();
                Some(ch)
            }
            _ => None,
        }
    }

    pub fn current_char_matches<F>(&mut self, f: F) -> bool
        where F: Fn(char) -> bool
    {

        match self.ch {
            Some(ch) if f(ch) => true,
            _ => false,
        }
    }

    pub fn peek_char_matches<F>(&mut self, f: F) -> bool
        where F: Fn(char) -> bool
    {

        match self.peek() {
            Some(ch) if f(ch) => {
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

        return self.current_char_matches(closure);
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
