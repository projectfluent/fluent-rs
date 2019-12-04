use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(Range<usize>),
    EqSign,
    CommentSign,
    GroupCommentSign,
    ResourceCommentSign,
    Eol(usize),
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
    End,
}

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
            buffer: None,
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

    fn tokenize_resource(&mut self, cc: u8) -> Token {
        match cc {
            b'-' => {
                self.ptr += 1;
                Token::MinusSign
            }
            b'#' => {
                let start = self.ptr;
                self.ptr += 1;
                while let Some(b'#') = self.source.get(self.ptr) {
                    self.ptr += 1;
                }
                self.state = LexerState::Comment;
                match self.ptr - start {
                    1 => return Token::CommentSign,
                    2 => return Token::GroupCommentSign,
                    3 => return Token::ResourceCommentSign,
                    _ => panic!(),
                }
            }
            b'\n' => {
                self.ptr += 1;
                return Token::Eol(self.ptr - 1);
            }
            b if b.is_ascii_alphabetic() => {
                self.state = LexerState::Message;
                self.get_ident()
            }
            _ => {
                panic!();
            }
        }
    }

    fn tokenize_message(&mut self, cc: u8) -> Option<Token> {
        if cc == b' ' {
            self.ptr += 1;
        }

        while let Some(cc) = self.source.get(self.ptr) {
            if *cc == b' ' {
                self.ptr += 1;
                continue;
            }

            match self.source.get(self.ptr) {
                Some(b'=') => {
                    self.state = LexerState::Pattern;
                    self.ptr += 1;
                    return Some(Token::EqSign);
                }
                Some(b'a'..=b'z') => {
                    return Some(self.get_ident());
                }
                None => return None,
                _ => {
                    // println!("{:#}", self.source[self.ptr]);
                    panic!()
                }
            }
        }
        None
    }

    fn tokenize_pattern(&mut self, cc: u8) -> Option<Token> {
        let indent_start = self.ptr;
        let mut in_indent = true;
        let mut start = self.ptr;
        let mut indent = 0;

        if cc == b' ' {
            self.ptr += 1;
        }

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
                        NextLine::TextContinuation => return Some(Token::Eol(self.ptr - 1)),
                        NextLine::Attribute => {
                            self.state = LexerState::Message;
                            self.buffer = Some(Token::Dot);
                            self.ptr += 1;
                            return Some(Token::Eot);
                        }
                        NextLine::NewEntry | NextLine::End => {
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

    fn tokenize_comment(&mut self, cc: u8) -> Option<Token> {
        if cc != b' ' {
            if cc != b'\n' {
                panic!();
            }
            self.state = LexerState::Resource;
            self.ptr += 1;
            return Some(Token::Text(0, self.ptr - 1..self.ptr - 1));
        }

        self.ptr += 1;

        let start = self.ptr;

        let mut end_vector = 0;

        while let Some(cc) = self.source.get(self.ptr) {
            self.ptr += 1;
            if *cc == b'\n' {
                end_vector = 1;
                break;
            }
        }
        self.state = LexerState::Resource;
        Some(Token::Text(0, start..(self.ptr - end_vector)))
    }

    fn check_next_line(&mut self) -> NextLine {
        let search_start = self.ptr;
        let mut indent_start = self.ptr;

        while let Some(cc) = self.source.get(self.ptr) {
            if *cc != b' ' {
                if *cc == b'\n' {
                    self.ptr += 1;
                    indent_start = self.ptr;
                } else {
                    break;
                }
            } else {
                self.ptr += 1;
            }
        }

        let start = self.ptr;
        let indent = start - indent_start;

        if indent == 0 {
            return NextLine::NewEntry;
        }

        match self.source.get(self.ptr) {
            Some(b'.') => NextLine::Attribute,
            _ => {
                self.ptr = search_start;
                NextLine::TextContinuation
            }
        }
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    // fn next(&mut self) -> Option<Self::Item> {
    //     if self.buffer.is_some() {
    //         return dbg!(self.buffer.take());
    //     }
    //     self.source.get(self.ptr).and_then(|cc| {
    //         let result = match self.state {
    //             LexerState::Resource => Some(self.tokenize_resource(*cc)),
    //             LexerState::Message => self.tokenize_message(*cc),
    //             LexerState::Pattern => self.tokenize_pattern(*cc),
    //             LexerState::Comment => self.tokenize_comment(*cc),
    //         };
    //         dbg!(result)
    //     })
    // }

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_some() {
            return self.buffer.take();
        }
        self.source.get(self.ptr).and_then(|cc| match self.state {
            LexerState::Resource => Some(self.tokenize_resource(*cc)),
            LexerState::Message => self.tokenize_message(*cc),
            LexerState::Pattern => self.tokenize_pattern(*cc),
            LexerState::Comment => self.tokenize_comment(*cc),
        })
    }
}
