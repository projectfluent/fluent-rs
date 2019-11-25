use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(Range<usize>),
    EqSign,
    CommentSign,
    Eol,
    Dot,
    Text(usize, Range<usize>),
}

#[derive(Debug, PartialEq)]
pub enum LexerState {
    Resource,
    Text,
}

#[derive(Debug)]
pub struct Lexer<'l> {
    pub state: LexerState,
    pub source: &'l [u8],
    pub length: usize,
    pub ptr: usize,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l [u8]) -> Self {
        let length = source.len();
        Lexer {
            state: LexerState::Resource,
            source,
            length,
            ptr: 0,
        }
    }

    fn get_ident(&mut self) -> Token {
        let start = self.ptr;
        self.ptr += 1;
        while let Some(cc) = self.source.get(self.ptr) {
            if !cc.is_ascii_alphanumeric() && *cc != b'-' && *cc != b'_' {
                break;
            }
            self.ptr += 1;
        }
        Token::Identifier(start..self.ptr)
    }

    fn tokenize_resource(&mut self) -> Option<Token> {
        while let Some(cc) = self.source.get(self.ptr) {
            if *cc != b' ' {
                break;
            }
            self.ptr += 1;
        }

        match self.source.get(self.ptr) {
            Some(cc) if cc.is_ascii_alphabetic() => Some(self.get_ident()),
            Some(b'=') => {
                self.state = LexerState::Text;
                self.ptr += 1;
                Some(Token::EqSign)
            }
            Some(b'#') => {
                self.state = LexerState::Text;
                self.ptr += 1;
                Some(Token::CommentSign)
            }
            Some(b'\n') => {
                self.ptr += 1;
                Some(Token::Eol)
            }
            Some(b'.') => {
                self.ptr += 1;
                Some(Token::Dot)
            }
            None => None,
            _ => {
                println!("{:#?}", self.source[self.ptr]);
                panic!()
            }
        }
    }

    fn tokenize_text(&mut self) -> Option<Token> {
        let indent_start = self.ptr;

        while let Some(cc) = self.source.get(self.ptr) {
            if *cc != b' ' {
                break;
            }
            self.ptr += 1;
        }

        let start = self.ptr;
        let mut new_line = false;
        while let Some(cc) = self.source.get(self.ptr) {
            if *cc == b'\n' {
                new_line = true;
                break;
            }
            self.ptr += 1;
        }
        self.state = LexerState::Resource;

        let indent = start - indent_start;

        if new_line {
            self.ptr += 1;
            Some(Token::Text(indent, start..self.ptr - 1))
        } else {
            Some(Token::Text(indent, start..self.ptr))
        }
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            LexerState::Resource => self.tokenize_resource(),
            LexerState::Text => self.tokenize_text(),
        }
    }
}
