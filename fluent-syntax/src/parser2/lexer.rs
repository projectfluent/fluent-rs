use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(Range<usize>),
    EqSign,
    CommentSign,
    Text(Range<usize>),
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
            if !cc.is_ascii_alphanumeric() && *cc != b'-' {
                break;
            }
            self.ptr += 1;
        }
        Token::Identifier(start..self.ptr)
    }

    fn get_text(&mut self) -> Token {
        let start = self.ptr;
        self.ptr += 1;
        self.state = LexerState::Resource;
        let mut new_line = false;
        while let Some(cc) = self.source.get(self.ptr) {
            if *cc == b'\n' {
                new_line = true;
                break;
            }
            self.ptr += 1;
        }

        if new_line {
            self.ptr += 1;
            Token::Text(start..self.ptr - 1)
        } else {
            Token::Text(start..self.ptr)
        }
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(cc) = self.source.get(self.ptr) {
            if *cc != b' ' && *cc != b'\n' {
                break;
            }
            self.ptr += 1;
        }

        match self.source.get(self.ptr) {
            Some(cc) if cc.is_ascii_alphabetic() => {
                if self.state == LexerState::Resource {
                    Some(self.get_ident())
                } else {
                    Some(self.get_text())
                }
            }
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
            None => None,
            _ => panic!(),
        }
    }
}
