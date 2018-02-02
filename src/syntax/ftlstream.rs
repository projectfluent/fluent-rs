use super::errors::ParserError;
use super::errors::ErrorKind;
use super::stream::ParserStream;
use super::parser::Result;

pub trait FTLParserStream<I> {
    fn skip_inline_ws(&mut self);
    fn peek_inline_ws(&mut self);
    fn skip_blank_lines(&mut self);
    fn peek_blank_lines(&mut self);
    fn skip_indent(&mut self);
    fn expect_char(&mut self, ch: char) -> Result<()>;
    fn expect_indent(&mut self) -> Result<()>;
    fn take_char_if(&mut self, ch: char) -> bool;

    fn take_char<F>(&mut self, f: F) -> Option<char>
    where
        F: Fn(char) -> bool;

    fn is_char_id_start(&mut self, ch: Option<char>) -> bool;
    fn is_entry_id_start(&mut self) -> bool;
    fn is_number_start(&mut self) -> bool;
    fn is_char_pattern_continuation(&self, ch: Option<char>) -> bool;
    fn is_peek_pattern_start(&mut self) -> bool;
    fn is_peek_next_line_zero_four_style_comment(&mut self) -> bool;
    fn is_peek_next_line_comment(&mut self, level: i8) -> bool;
    fn is_peek_next_line_variant_start(&mut self) -> bool;
    fn is_peek_next_line_attribute_start(&mut self) -> bool;
    fn is_peek_next_line_pattern_start(&mut self) -> bool;
    fn skip_to_next_entry_start(&mut self);
    fn take_id_start(&mut self, allow_private: bool) -> Result<char>;
    fn take_id_char(&mut self) -> Option<char>;
    fn take_variant_name_char(&mut self) -> Option<char>;
    fn take_digit(&mut self) -> Option<char>;
}

static INLINE_WS: [char; 2] = [' ', '\t'];
static SPECIAL_LINE_START_CHARS: [char; 4] = ['}', '.', '[', '*'];

impl<I> FTLParserStream<I> for ParserStream<I>
where
    I: Iterator<Item = char>,
{
    fn skip_inline_ws(&mut self) {
        while let Some(ch) = self.ch {
            if !INLINE_WS.contains(&ch) {
                break;
            }
            self.next();
        }
    }

    fn peek_inline_ws(&mut self) {
        while let Some(ch) = self.current_peek() {
            if !INLINE_WS.contains(&ch) {
                break;
            }
            self.peek();
        }
    }

    fn skip_blank_lines(&mut self) {
        loop {
            self.peek_inline_ws();

            if self.current_peek() == Some('\n') {
                self.skip_to_peek();
                self.next();
            } else {
                self.reset_peek(None);
                break;
            }
        }
    }

    fn peek_blank_lines(&mut self) {
        loop {
            let line_start = self.get_peek_index();

            self.peek_inline_ws();

            if self.current_peek_is('\n') {
                self.peek();
            } else {
                self.reset_peek(Some(line_start));
                break;
            }
        }
    }

    fn skip_indent(&mut self) {
        self.skip_blank_lines();
        self.skip_inline_ws();
    }

    fn expect_char(&mut self, ch: char) -> Result<()> {
        if self.ch == Some(ch) {
            self.next();
            return Ok(());
        }

        if self.ch == Some('\n') {
            // Unicode Character 'SYMBOL FOR NEWLINE' (U+2424)
            return error!(ErrorKind::ExpectedToken { token: '\u{2424}' });
        }

        error!(ErrorKind::ExpectedToken { token: ch })
    }

    fn expect_indent(&mut self) -> Result<()> {
        self.expect_char('\n')?;
        self.skip_blank_lines();
        self.expect_char(' ')?;
        self.skip_inline_ws();
        Ok(())
    }

    fn take_char_if(&mut self, ch: char) -> bool {
        if self.ch == Some(ch) {
            self.next();
            return true;
        }

        false
    }

    fn take_char<F>(&mut self, f: F) -> Option<char>
    where
        F: Fn(char) -> bool,
    {
        if let Some(ch) = self.ch {
            if f(ch) {
                self.next();
                return Some(ch);
            }
        }
        None
    }

    fn is_char_id_start(&mut self, ch: Option<char>) -> bool {
        match ch {
            Some('a'...'z') | Some('A'...'Z') => true,
            _ => false,
        }
    }

    fn is_entry_id_start(&mut self) -> bool {
        if let Some('-') = self.ch {
            self.peek();
        }
        let ch = self.current_peek();
        let is_id = self.is_char_id_start(ch);
        self.reset_peek(None);
        is_id
    }

    fn is_number_start(&mut self) -> bool {
        if let Some('-') = self.ch {
            self.peek();
        }
        let ch = self.current_peek();
        let is_digit = match ch {
            Some('0'...'9') => true,
            _ => false,
        };
        self.reset_peek(None);
        is_digit
    }

    fn is_char_pattern_continuation(&self, ch: Option<char>) -> bool {
        match ch {
            Some(ch) => !SPECIAL_LINE_START_CHARS.contains(&ch),
            _ => false,
        }
    }

    fn is_peek_pattern_start(&mut self) -> bool {
        self.peek_inline_ws();

        if let Some(ch) = self.current_peek() {
            if ch != '\n' {
                return true;
            }
        }

        return self.is_peek_next_line_pattern_start();
    }

    fn is_peek_next_line_zero_four_style_comment(&mut self) -> bool {
        if !self.current_peek_is('\n') {
            return false;
        }
        self.peek();

        if self.current_peek_is('/') {
            self.peek();
            if self.current_peek_is('/') {
                self.reset_peek(None);
                return true;
            }
        }
        self.reset_peek(None);
        return false;
    }

    fn is_peek_next_line_comment(&mut self, level: i8) -> bool {
        if !self.current_peek_is('\n') {
            return false;
        }

        let mut i = 0;

        while i <= level && (level == -1 && i < 3) {
            self.peek();
            if !self.current_peek_is('#') {
                if i != level && level != -1 {
                    self.reset_peek(None);
                    return false;
                }
                break;
            }
            i += 1;
        }

        self.peek();

        if let Some(ch) = self.current_peek() {
            if [' ', '\n'].contains(&ch) {
                self.reset_peek(None);
                return true;
            }
        }
        self.reset_peek(None);
        return false;
    }

    fn is_peek_next_line_variant_start(&mut self) -> bool {
        if !self.current_peek_is('\n') {
            return false;
        }
        self.peek();

        self.peek_blank_lines();

        let ptr = self.get_peek_index();

        self.peek_inline_ws();

        if self.get_peek_index() - ptr == 0 {
            self.reset_peek(None);
            return false;
        }

        if self.current_peek_is('*') {
            self.peek();
        }

        if self.current_peek_is('[') && !self.peek_char_is('[') {
            self.reset_peek(None);
            return true;
        }
        self.reset_peek(None);
        false
    }

    fn is_peek_next_line_attribute_start(&mut self) -> bool {
        if !self.current_peek_is('\n') {
            return false;
        }
        self.peek();

        self.peek_blank_lines();

        let ptr = self.get_peek_index();

        self.peek_inline_ws();

        if self.get_peek_index() - ptr == 0 {
            self.reset_peek(None);
            return false;
        }

        if self.current_peek_is('.') {
            self.reset_peek(None);
            return true;
        }

        self.reset_peek(None);
        false
    }

    fn is_peek_next_line_pattern_start(&mut self) -> bool {
        if !self.current_peek_is('\n') {
            return false;
        }
        self.peek();

        self.peek_blank_lines();

        let ptr = self.get_peek_index();

        self.peek_inline_ws();

        if self.get_peek_index() - ptr == 0 {
            self.reset_peek(None);
            return false;
        }

        if !self.is_char_pattern_continuation(self.current_peek()) {
            self.reset_peek(None);
            return false;
        }

        self.reset_peek(None);
        true
    }

    fn skip_to_next_entry_start(&mut self) {
        while let Some(_) = self.next() {
            if self.current_is('\n') && !self.peek_char_is('\n') {
                self.next();

                if self.ch.is_none() || self.is_entry_id_start() || self.current_is('#')
                    || (self.current_is('/') && self.peek_char_is('/'))
                    || (self.current_is('[') && self.peek_char_is('['))
                {
                    break;
                }
            }
        }
    }

    fn take_id_start(&mut self, allow_term: bool) -> Result<char> {
        if allow_term && self.current_is('-') {
            self.next();
            return Ok('-');
        }

        if let Some(ch) = self.ch {
            if self.is_char_id_start(Some(ch)) {
                let ret = self.ch.unwrap();
                self.next();
                return Ok(ret);
            }
        }

        let allowed_range = if allow_term {
            "'a'...'z' | 'A'...'Z' | '-'"
        } else {
            "'a'...'z' | 'A'...'Z'"
        };
        error!(ErrorKind::ExpectedCharRange {
            range: String::from(allowed_range),
        })
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

    fn take_variant_name_char(&mut self) -> Option<char> {
        let closure = |x| match x {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' | '-' | ' ' => true,
            _ => false,
        };

        match self.take_char(closure) {
            Some(ch) => Some(ch),
            None => None,
        }
    }

    fn take_digit(&mut self) -> Option<char> {
        let closure = |x| match x {
            '0'...'9' => true,
            _ => false,
        };

        match self.take_char(closure) {
            Some(ch) => Some(ch),
            None => None,
        }
    }
}
