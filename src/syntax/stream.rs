extern crate itertools;

use super::errors::ParserError;
use self::itertools::MultiPeek;

use std::result;

type Result<T> = result::Result<T, ParserError>;

pub trait ParserStream<I> {
    fn peek_line_ws(&mut self) -> (u8, Option<char>);
    fn skip_line_ws(&mut self);
    fn skip_ws(&mut self);
    fn skip_ws_lines(&mut self);
    fn expect_char(&mut self, ch: char) -> Result<()>;
    fn take_char_if(&mut self, ch: char) -> bool;
    fn take_char_after_line_ws_if(&mut self, ch: char) -> bool;
    fn take_char<F>(&mut self, f: F) -> Option<char> where F: Fn(char) -> bool;
    fn peek_char_matches<F>(&mut self, f: F) -> bool where F: Fn(char) -> bool;
    fn is_id_start(&mut self) -> bool;
    fn take_id_start(&mut self) -> Option<char>;
    fn take_id_char(&mut self) -> Option<char>;
    fn take_kw_char(&mut self) -> Option<char>;
}

impl<I> ParserStream<I> for MultiPeek<I>
    where I: Iterator<Item = char>
{
    fn peek_line_ws(&mut self) -> (u8, Option<char>) {
        let mut i: u8 = 0;
        while let Some(&ch) = self.peek() {
            if ch != ' ' && ch != '\t' {
                return (i, Some(ch));
            }

            i += 1;
        }
        return (i, None);
    }

    fn skip_line_ws(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch != ' ' && ch != '\t' {
                self.reset_peek();
                break;
            }

            self.next();
        }
    }

    fn skip_ws(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch != ' ' && ch != '\n' && ch != '\t' && ch != '\r' {
                self.reset_peek();
                break;
            }

            self.next();
        }
    }

    fn skip_ws_lines(&mut self) {
        loop {
            let (wc, ch) = self.peek_line_ws();

            match ch {
                Some('\n') => {
                    for _ in 0..wc {
                        self.next();
                    }
                    self.next();
                }
                _ => {
                    self.reset_peek();
                    break;
                }
            }
        }
    }

    fn expect_char(&mut self, ch: char) -> Result<()> {
        match self.next() {
            Some(ch2) if ch == ch2 => Ok(()),
            _ => Err(ParserError::Generic),
        }
    }

    fn take_char_if(&mut self, ch: char) -> bool {
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

    fn take_char_after_line_ws_if(&mut self, ch2: char) -> bool {
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

    fn take_char<F>(&mut self, f: F) -> Option<char>
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

    fn peek_char_matches<F>(&mut self, f: F) -> bool
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

    fn is_id_start(&mut self) -> bool {
        let closure = |x| match x {
            'a'...'z' | 'A'...'Z' | '_' => true,
            _ => false,
        };

        return self.peek_char_matches(closure);
    }

    fn take_id_start(&mut self) -> Option<char> {
        let closure = |x| match x {
            'a'...'z' | 'A'...'Z' | '_' => true,
            _ => false,
        };

        match self.take_char(closure) {
            Some(ch) => Some(ch),
            None => None,
        }
    }

    fn take_id_char(&mut self) -> Option<char> {
        let closure = |x| match x {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' | '-' => true,
            _ => false,
        };

        match self.take_char(closure) {
            Some(ch) => Some(ch),
            None => None,
        }
    }

    fn take_kw_char(&mut self) -> Option<char> {
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
