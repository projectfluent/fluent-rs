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
    Message, // Or Term
    Pattern,
    Comment,
}

#[derive(Debug, PartialEq)]
pub enum NextLine {
    TextContinuation,
    Attribute,
    NewEntry,
}

#[derive(Debug)]
pub struct Lexer<'l> {
    pub state: LexerState,
    pub source: &'l [u8],
    pub length: usize,
    pub ptr: usize,
    pub buffer: Option<Token>,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l [u8]) -> Self {
        let length = source.len();
        Lexer {
            state: LexerState::Resource,
            source,
            length,
            ptr: 0,
            buffer: None
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

    fn tokenize_message(&mut self) -> Option<Token> {
        let start = self.ptr;
        while let Some(cc) = self.source.get(self.ptr) {
            if *cc != b' ' {
                break;
            }
            self.ptr += 1;
        }
        let indent = self.ptr - start;

        match self.source.get(self.ptr) {
            Some(b'=') => {
                self.state = LexerState::Pattern;
                self.ptr += 1;
                Some(Token::EqSign)
            },
            Some(b'.') if indent > 0 => {
                self.ptr += 1;
                Some(Token::Dot)
            },
            Some(cc) if cc.is_ascii_alphabetic() => {
                Some(self.get_ident())
            },
            None => None,
            _ => {
                println!("{:#}", self.source[self.ptr]);
                panic!()
            },
        }
    }

    fn tokenize_resource(&mut self) -> Option<Token> {
        match self.source.get(self.ptr) {
            Some(cc) if cc.is_ascii_alphabetic() => {
                self.state = LexerState::Message;
                Some(self.get_ident())
            },
            Some(b'-') => {
                self.ptr += 1;
                Some(Token::MinusSign)
            },
            Some(b'#') => {
                let start = self.ptr;
                self.ptr += 1;
                while let Some(b'#') = self.source.get(self.ptr) {
                    self.ptr += 1;
                }
                self.state = LexerState::Comment;
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
            None => None,
            _ => panic!()
        }
    }

    fn tokenize_comment(&mut self) -> Option<Token> {
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

    fn tokenize_pattern(&mut self) -> Option<Token> {
        let indent_start = self.ptr;
        let mut in_indent = true;
        let mut start = self.ptr;
        let mut indent = 0;

        while let Some(cc) = self.source.get(self.ptr) {
            if in_indent {
                if *cc == b' ' {
                    self.ptr += 1;
                    continue;
                } else {
                    start = self.ptr;
                    indent = start - indent_start;
                    in_indent = false;
                }
            }
            match cc {
                b'\n' if start == self.ptr => {
                    self.ptr += 1;
                    match self.check_next_line() {
                        NextLine::TextContinuation => return Some(Token::Eol),
                        NextLine::Attribute => {
                            self.state = LexerState::Message;
                            self.buffer = Some(Token::Dot);
                            self.ptr += 1;
                            return Some(Token::Eot);
                        },
                        NextLine::NewEntry => {
                            self.state = LexerState::Resource;
                            return Some(Token::Eot);
                        }
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

    fn check_next_line(&mut self) -> NextLine {
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
            return NextLine::NewEntry;
        }

        match self.source.get(self.ptr) {
            Some(b'.') => {
                NextLine::Attribute
            }
            _ => NextLine::TextContinuation,
        }
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.buffer.take() {
            return Some(token);
        }
        let x = match self.state {
            LexerState::Resource => self.tokenize_resource(),
            LexerState::Message => self.tokenize_message(),
            LexerState::Pattern => self.tokenize_pattern(),
            LexerState::Comment => self.tokenize_comment(),
        };
        x
    }
}
