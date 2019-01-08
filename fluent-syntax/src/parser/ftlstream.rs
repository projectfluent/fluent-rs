use super::errors::{ErrorKind, ParserError};
use super::Result;
use std::str;

pub struct ParserStream<'p> {
    pub source: &'p [u8],
    pub length: usize,
    pub ptr: usize,
}

impl<'p> ParserStream<'p> {
    pub fn new(stream: &'p str) -> Self {
        ParserStream {
            source: stream.as_bytes(),
            length: stream.len(),
            ptr: 0,
        }
    }

    pub fn is_current_byte(&self, b: u8) -> bool {
        if self.ptr >= self.length {
            return false;
        }
        self.source[self.ptr] == b
    }

    pub fn is_byte_at(&self, b: u8, pos: usize) -> bool {
        if pos >= self.length {
            return false;
        }
        self.source[pos] == b
    }

    pub fn expect_byte(&mut self, b: u8) -> Result<()> {
        if !self.is_current_byte(b) {
            return error!(ErrorKind::ExpectedToken(b as char), self.ptr);
        }
        self.ptr += 1;
        Ok(())
    }

    pub fn take_if(&mut self, b: u8) -> bool {
        if self.is_current_byte(b) {
            self.ptr += 1;
            true
        } else {
            false
        }
    }

    pub fn skip_blank_block(&mut self) -> usize {
        let mut count = 0;
        loop {
            let start = self.ptr;
            self.skip_blank_inline();
            if !self.skip_eol() {
                self.ptr = start;
                break;
            }
            count += 1;
        }
        count
    }

    pub fn skip_blank(&mut self) {
        while self.ptr < self.length {
            let b = self.source[self.ptr];
            if b == b' ' || b == b'\n' {
                self.ptr += 1;
            } else {
                break;
            }
        }
    }

    pub fn skip_blank_inline(&mut self) -> usize {
        let start = self.ptr;
        while self.ptr < self.length {
            let b = self.source[self.ptr];
            if b == b' ' {
                self.ptr += 1;
            } else {
                break;
            }
        }
        self.ptr - start
    }

    pub fn skip_to_next_entry_start(&mut self) {
        while self.ptr < self.length {
            if (self.ptr == 0 || self.is_byte_at(b'\n', self.ptr - 1))
                && (self.is_identifier_start()
                    || self.is_current_byte(b'-')
                    || self.is_current_byte(b'#'))
            {
                break;
            }

            self.ptr += 1;

            while self.ptr < self.length && !self.is_byte_at(b'\n', self.ptr - 1) {
                self.ptr += 1;
            }
        }
    }

    pub fn skip_eol(&mut self) -> bool {
        if self.ptr >= self.length {
            return false;
        }

        if self.is_current_byte(b'\n') {
            self.ptr += 1;
            return true;
        }

        if self.is_current_byte(b'\r') && self.is_byte_at(b'\n', self.ptr + 1) {
            self.ptr += 2;
            return true;
        }
        false
    }

    pub fn skip_to_value_start(&mut self) -> Option<bool> {
        self.skip_blank_inline();

        if !self.is_eol() {
            return Some(true);
        }

        self.skip_blank_block();

        let inline = self.skip_blank_inline();

        if self.is_current_byte(b'{') {
            return Some(true);
        }

        if inline == 0 {
            return None;
        }

        if !self.is_char_pattern_continuation() {
            return None;
        }
        self.ptr -= inline;
        Some(false)
    }

    pub fn skip_unicode_escape_sequence(&mut self, length: usize) -> Result<()> {
        let start = self.ptr;
        for _ in 0..length {
            match self.source[self.ptr] {
                b'0'...b'9' => self.ptr += 1,
                b'a'...b'f' => self.ptr += 1,
                b'A'...b'F' => self.ptr += 1,
                _ => break,
            }
        }
        if self.ptr - start != length {
            return error!(
                ErrorKind::InvalidUnicodeEscapeSequence(
                    self.get_slice(start, self.ptr + 1).to_owned()
                ),
                self.ptr
            );
        }
        Ok(())
    }

    pub fn is_char_pattern_continuation(&self) -> bool {
        if self.ptr >= self.length {
            return false;
        }

        let b = self.source[self.ptr];
        b != b'}' && b != b'.' && b != b'[' && b != b'*'
    }

    pub fn is_identifier_start(&self) -> bool {
        if self.ptr >= self.length {
            return false;
        }
        let b = self.source[self.ptr];
        (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z')
    }

    pub fn is_number_start(&self) -> bool {
        if self.ptr >= self.length {
            return false;
        }
        let b = self.source[self.ptr];
        b == b'-' || (b >= b'0' && b <= b'9')
    }

    pub fn is_eol(&self) -> bool {
        if self.is_current_byte(b'\n') {
            return true;
        }

        self.is_current_byte(b'\r') && self.is_byte_at(b'\n', self.ptr + 1)
    }

    pub fn get_slice(&self, start: usize, end: usize) -> &'p str {
        unsafe { str::from_utf8_unchecked(&self.source[start..end]) }
    }

    pub fn skip_digits(&mut self) -> Result<()> {
        let start = self.ptr;
        while let Some(b) = self.source.get(self.ptr) {
            if b >= &b'0' && b <= &b'9' {
                self.ptr += 1;
            } else {
                break;
            }
        }
        if start == self.ptr {
            error!(
                ErrorKind::ExpectedCharRange {
                    range: "0-9".to_string()
                },
                self.ptr
            )
        } else {
            Ok(())
        }
    }
}
