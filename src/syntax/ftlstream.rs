use super::errors::ParserError;
use super::errors::ErrorKind;
use super::stream::ParserStream;
use super::parser::Result;

pub trait FTLParserStream<I> {
    fn peek_line_ws(&mut self);
    fn skip_ws_lines(&mut self);
    fn skip_line_ws(&mut self);
    fn expect_char(&mut self, ch: char) -> Result<()>;
    fn take_char_if(&mut self, ch: char) -> bool;

    fn take_char<F>(&mut self, f: F) -> Option<char>
    where
        F: Fn(char) -> bool;

    fn is_char_id_start(&mut self, ch: Option<char>) -> bool;
    fn is_message_id_start(&mut self) -> bool;
    fn is_peek_next_line_indented(&mut self) -> bool;
    fn is_peek_next_line_variant_start(&mut self) -> bool;
    fn is_peek_next_line_attribute_start(&mut self) -> bool;
    fn is_peek_next_line_pattern(&mut self) -> bool;
    fn skip_to_next_entry_start(&mut self);
    fn take_id_start(&mut self, allow_private: bool) -> Result<char>;
    fn take_id_char(&mut self) -> Option<char>;
    fn take_symb_char(&mut self) -> Option<char>;
    fn take_digit(&mut self) -> Option<char>;
}

impl<I> FTLParserStream<I> for ParserStream<I>
where
    I: Iterator<Item = char>,
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

            if self.current_peek() == Some('\n') {
                self.skip_to_peek();
                self.next();
            } else {
                self.reset_peek();
                break;
            }
        }
    }

    fn skip_line_ws(&mut self) {
        while self.ch == Some(' ') || self.ch == Some('\t') {
            self.next();
        }
    }

    fn expect_char(&mut self, ch: char) -> Result<()> {
        if self.ch == Some(ch) {
            self.next();
            return Ok(());
        }

        error!(ErrorKind::ExpectedToken { token: ch })
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

    fn is_message_id_start(&mut self) -> bool {
        if let Some('-') = self.ch {
            self.peek();
        }
        let ch = self.current_peek();
        let is_id = self.is_char_id_start(ch);
        self.reset_peek();
        is_id
    }

    fn is_peek_next_line_indented(&mut self) -> bool {
        if !self.current_peek_is('\n') {
            return false;
        }
        self.peek();

        if self.current_peek_is(' ') {
            self.reset_peek();
            return true;
        }

        self.reset_peek();
        false
    }

    fn is_peek_next_line_variant_start(&mut self) -> bool {
        if !self.current_peek_is('\n') {
            return false;
        }
        self.peek();

        let ptr = self.get_peek_index();

        self.peek_line_ws();

        if self.get_peek_index() - ptr == 0 {
            self.reset_peek();
            return false;
        }

        if self.current_peek_is('*') {
            self.peek();
        }

        if self.current_peek_is('[') && !self.peek_char_is('[') {
            self.reset_peek();
            return true;
        }
        self.reset_peek();
        false
    }

    fn is_peek_next_line_attribute_start(&mut self) -> bool {
        if !self.current_peek_is('\n') {
            return false;
        }
        self.peek();

        let ptr = self.get_peek_index();

        self.peek_line_ws();

        if self.get_peek_index() - ptr == 0 {
            self.reset_peek();
            return false;
        }

        if self.current_peek_is('.') {
            self.reset_peek();
            return true;
        }

        self.reset_peek();
        false
    }

    fn is_peek_next_line_pattern(&mut self) -> bool {
        if !self.current_peek_is('\n') {
            return false;
        }
        self.peek();

        let ptr = self.get_peek_index();

        self.peek_line_ws();

        if self.get_peek_index() - ptr == 0 {
            self.reset_peek();
            return false;
        }

        if self.current_peek_is('}') || self.current_peek_is('.') || self.current_peek_is('[')
            || self.current_peek_is('*')
        {
            self.reset_peek();
            return false;
        }

        self.reset_peek();
        true
    }

    fn skip_to_next_entry_start(&mut self) {
        while let Some(_) = self.next() {
            if self.current_is('\n') && !self.peek_char_is('\n')
                && (self.next() == None || self.is_message_id_start() || self.current_is('#'))
            {
                break;
            }
        }
    }

    fn take_id_start(&mut self, allow_private: bool) -> Result<char> {
        let closure = |x| match x {
            'a'...'z' | 'A'...'Z' => true,
            '-' => allow_private,
            _ => false,
        };

        if let Some(ch) = self.take_char(closure) {
            Ok(ch)
        } else if allow_private {
            error!(ErrorKind::ExpectedCharRange {
                range: String::from("'a'...'z' | 'A'...'Z'"),
            })
        } else {
            error!(ErrorKind::ExpectedCharRange {
                range: String::from("'a'...'z' | 'A'...'Z' | '-'"),
            })
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

    fn take_symb_char(&mut self) -> Option<char> {
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
