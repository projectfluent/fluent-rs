use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(Range<usize>),
    EqSign,
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
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;
    
    // Iterator over u8?
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(b' ') = self.source.get(self.ptr) {
            self.ptr += 1;
        }
        match self.source.get(self.ptr) {
            Some(cc) if cc.is_ascii_alphabetic() => {
                self.ptr += 1;
                if self.state == LexerState::Resource {
                    let start = self.ptr;
                    while let Some(cc) = self.source.get(self.ptr) {
                        if cc == &b' ' {
                            break;
                        }
                        self.ptr += 1;
                    }
                    Some(Token::Identifier(start .. self.ptr))
                } else {
                    let start = self.ptr;
                    self.state = LexerState::Resource;
                    let mut new_line = false;
                    while let Some(cc) = self.source.get(self.ptr) {
                        if cc == &b'\n' {
                            new_line = true;
                            break;
                        }
                        self.ptr += 1;
                    }

                    if new_line {
                        self.ptr += 1;
                        Some(Token::Text(start .. self.ptr - 1))
                    } else {
                        Some(Token::Text(start .. self.ptr))
                    }
                }
            },
            Some(b'=') => {
                self.state = LexerState::Text;
                self.ptr += 1;
                Some(Token::EqSign)
            },
            None => None,
            _ => {
                panic!()
            }
        }
    }
}
