use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(Range<usize>),
    EqSign,
    CommentSign,
    GroupCommentSign,
    ResourceCommentSign,
    Eol,
    Eot, // End Of Text
    Dot,
    MinusSign,
    Text(usize, Range<usize>),
}

#[derive(Debug, PartialEq)]
pub enum LexerState {
    Resource,
    Text,
    TextLine,
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
                let start = self.ptr;
                self.ptr += 1;
                while let Some(b'#') = self.source.get(self.ptr) {
                    self.ptr += 1;
                }
                self.state = LexerState::TextLine;
                match self.ptr - start {
                    1 => Some(Token::CommentSign),
                    2 => Some(Token::GroupCommentSign),
                    3 => Some(Token::ResourceCommentSign),
                    _ => panic!(),
                }
            }
            Some(b'\n') => {
                self.ptr += 1;
                Some(Token::Eol)
            }
            Some(b'.') => {
                self.ptr += 1;
                Some(Token::Dot)
            }
            Some(b'-') => {
                self.ptr += 1;
                Some(Token::MinusSign)
            }
            None => None,
            _ => {
                println!("{:#?}", self.source[self.ptr]);
                panic!()
            }
        }
    }

    fn tokenize_text_line(&mut self) -> Option<Token> {
        let indent_start = self.ptr;

        while let Some(cc) = self.source.get(self.ptr) {
            if *cc != b' ' {
                break;
            }
            self.ptr += 1;
        }

        let start = self.ptr;
        let indent = start - indent_start;

        while let Some(cc) = self.source.get(self.ptr) {
            if *cc == b'\n' {
                self.ptr += 1;
                break;
            }
            self.ptr += 1;
        }
        self.state = LexerState::Resource;
        Some(Token::Text(indent, start..self.ptr - 1))
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
        let indent = start - indent_start;

        while let Some(cc) = self.source.get(self.ptr) {
            match cc {
                b'\n' if start == self.ptr => {
                    self.ptr += 1;
                    if self.try_if_line_is_text() {
                        return Some(Token::Eol);
                    } else {
                        self.state = LexerState::Resource;
                        return Some(Token::Eot);
                    }
                }
                b'\n' => {
                    return Some(Token::Text(indent, start..self.ptr));
                }
                _ => {}
            }
            self.ptr += 1;
        }
        None
    }

    fn try_if_line_is_text(&mut self) -> bool {
        let indent_start = self.ptr;

        while let Some(cc) = self.source.get(self.ptr) {
            if *cc != b' ' {
                break;
            }
            self.ptr += 1;
        }

        let start = self.ptr;
        let indent = start - indent_start;

        if indent == 0 {
            return false;
        }

        match self.source.get(self.ptr) {
            Some(b'.') => {
                self.ptr -= 1;
                false
            }
            _ => true,
        }
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            LexerState::Resource => self.tokenize_resource(),
            LexerState::Text => self.tokenize_text(),
            LexerState::TextLine => self.tokenize_text_line(),
        }
    }
}
