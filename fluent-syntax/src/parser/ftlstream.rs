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
        self.source.get(self.ptr) == Some(&b)
    }

    pub fn is_byte_at(&self, b: u8, pos: usize) -> bool {
        self.source.get(pos) == Some(&b)
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
        loop {
            match self.source.get(self.ptr) {
                Some(b) if [b' ', b'\n'].contains(b) => self.ptr += 1,
                _ => break,
            }
        }
    }

    pub fn skip_blank_inline(&mut self) -> usize {
        let start = self.ptr;
        while let Some(b' ') = self.source.get(self.ptr) {
            self.ptr += 1;
        }
        self.ptr - start
    }

    pub fn skip_to_next_entry_start(&mut self) {
        while let Some(b) = self.source.get(self.ptr) {
            let new_line = self.ptr == 0 || self.source.get(self.ptr - 1) == Some(&b'\n');

            if new_line && (self.is_byte_alphabetic(*b) || [b'-', b'#'].contains(b)) {
                break;
            }

            self.ptr += 1;
        }
    }

    pub fn skip_eol(&mut self) -> bool {
        match self.source.get(self.ptr) {
            Some(b'\n') => {
                self.ptr += 1;
                true
            }
            Some(b'\r') if self.is_byte_at(b'\n', self.ptr + 1) => {
                self.ptr += 2;
                true
            }
            _ => false,
        }
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
            match self.source.get(self.ptr) {
                Some(b) if b.is_ascii_hexdigit() => self.ptr += 1,
                _ => break,
            }
        }
        if self.ptr - start != length {
            let end = if self.ptr >= self.length {
                self.ptr
            } else {
                self.ptr + 1
            };
            return error!(
                ErrorKind::InvalidUnicodeEscapeSequence(self.get_slice(start, end).to_owned()),
                self.ptr
            );
        }
        Ok(())
    }

    pub fn is_char_pattern_continuation(&self) -> bool {
        match self.source.get(self.ptr) {
            Some(b) => self.is_byte_pattern_continuation(*b),
            _ => false,
        }
    }

    pub fn is_identifier_start(&self) -> bool {
        match self.source.get(self.ptr) {
            Some(b) if self.is_byte_alphabetic(*b) => true,
            _ => false,
        }
    }

    pub fn is_byte_alphabetic(&self, b: u8) -> bool {
        (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z')
    }

    pub fn is_byte_digit(&self, b: u8) -> bool {
        b >= b'0' && b <= b'9'
    }

    pub fn is_byte_pattern_continuation(&self, b: u8) -> bool {
        ![b'}', b'.', b'[', b'*'].contains(&b)
    }

    pub fn is_number_start(&self) -> bool {
        match self.source.get(self.ptr) {
            Some(b) if (b == &b'-') || self.is_byte_digit(*b) => true,
            _ => false,
        }
    }

    pub fn is_eol(&self) -> bool {
        match self.source.get(self.ptr) {
            Some(b'\n') => true,
            Some(b'\r') if self.is_byte_at(b'\n', self.ptr + 1) => true,
            _ => false,
        }
    }

    pub fn get_slice(&self, start: usize, end: usize) -> &'p str {
        unsafe { str::from_utf8_unchecked(&self.source[start..end]) }
    }

    pub fn skip_digits(&mut self) -> Result<()> {
        let start = self.ptr;
        loop {
            match self.source.get(self.ptr) {
                Some(b) if self.is_byte_digit(*b) => self.ptr += 1,
                _ => break,
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
