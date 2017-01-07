use super::errors::ParserError;
use super::errors::ErrorKind;
use super::iter::ParserStream;

use std::result;

type Result<T> = result::Result<T, ParserError>;

macro_rules! error {
    ($kind:expr) => {{
        Err(ParserError {
            info: None,
            kind: $kind
        })
    }};
}

pub trait FTLParserStream<I> {
    fn peek_line_ws(&mut self);
    fn skip_ws_lines(&mut self);
    fn skip_line_ws(&mut self);
    fn skip_ws(&mut self);
    fn expect_char(&mut self, ch: char) -> Result<()>;
    fn take_char_if(&mut self, ch: char) -> bool;

    fn take_char<F>(&mut self, f: F) -> Option<char> where F: Fn(char) -> bool;

    fn is_id_start(&mut self) -> bool;
    fn is_peek_next_line_member_start(&mut self) -> bool;
    fn skip_to_next_entry_start(&mut self);
    fn take_id_start(&mut self) -> Result<char>;
    fn take_id_char(&mut self) -> Option<char>;
    fn take_kw_char(&mut self) -> Option<char>;
}

impl<I> FTLParserStream<I> for ParserStream<I>
    where I: Iterator<Item = char>
{
    fn peek_line_ws(&mut self) {
        while let Some(ch) = self.current_peek() {
            if ch != ' ' && ch != '\t' {
                break;
            }

            self.peek();
        }
    }

    fn skip_ws_lines(&mut self) {
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

    fn skip_line_ws(&mut self) {
        while let Some(ch) = self.ch {
            if ch != ' ' && ch != '\t' {
                break;
            }

            self.next();
        }
    }

    fn skip_ws(&mut self) {
        while let Some(ch) = self.ch {
            if ch != ' ' && ch != '\n' && ch != '\t' && ch != '\r' {
                break;
            }

            self.next();
        }
    }

    fn expect_char(&mut self, ch: char) -> Result<()> {
        match self.ch {
            Some(ch2) if ch == ch2 => {
                self.next();
                Ok(())
            }
            _ => error!(ErrorKind::ExpectedToken { token: ch }),
        }
    }

    fn take_char_if(&mut self, ch: char) -> bool {
        match self.ch {
            Some(ch2) if ch == ch2 => {
                self.next();
                true
            }
            _ => false,
        }
    }

    fn take_char<F>(&mut self, f: F) -> Option<char>
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

    fn is_id_start(&mut self) -> bool {
        match self.ch {
            Some(ch) => {
                match ch {
                    'a'...'z' | 'A'...'Z' | '_' => true,
                    _ => false,
                }
            }
            None => false,
        }
    }

    fn is_peek_next_line_member_start(&mut self) -> bool {
        if !self.current_peek_is('\n') {
            return false;
        }
        self.peek();

        self.peek_line_ws();

        if self.current_peek_is('*') {
            self.peek();
        }

        if self.current_peek_is('[') && !self.peek_char_is('[') {
            self.reset_peek();
            return true;
        }
        self.reset_peek();
        return false;
    }

    fn skip_to_next_entry_start(&mut self) {
        loop {
            if self.current_is('\n') {
                if !self.peek_char_is('\n') {
                    match self.next() {
                        Some(ch) => {
                            match ch {
                                'a'...'z' | 'A'...'Z' | '_' => break,
                                '#' => break,
                                '[' => {
                                    if self.peek_char_is('[') {
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
            }

            if self.next().is_none() {
                break;
            }
        }
    }

    fn take_id_start(&mut self) -> Result<char> {
        let closure = |x| match x {
            'a'...'z' | 'A'...'Z' | '_' => true,
            _ => false,
        };

        match self.take_char(closure) {
            Some(ch) => Ok(ch),
            None => {
                error!(ErrorKind::ExpectedCharRange {
                    range: String::from("'a'...'z' | 'A'...'Z' | '_'"),
                })
            }
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
