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
    Junk(Range<usize>),
    OpenCurlyBraces,
    CloseCurlyBraces,
    DoubleQuote,
    Number(Range<usize>),
}

#[derive(Debug, PartialEq)]
pub enum LexerState {
    Resource,
    Message,
    // Term,
    Pattern,
    Comment,
    Expression,
    StringLiteral,
}

#[derive(Debug, PartialEq)]
pub enum NextLine {
    TextContinuation,
    Attribute,
    NewEntry,
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    Unknown,
}

type LexerResult = Result<Token, LexerError>;
type LexerOptionResult = Result<Option<Token>, LexerError>;

pub struct Lexer<'l> {
    pub state: LexerState,
    pub source: &'l [u8],
    pub ptr: usize,
    pub entry_start: usize,
    pub buffer: Option<Token>,
    pub peek: Option<Token>,
    pub next_ptr: usize,
    pub done: bool,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l [u8]) -> Self {
        Lexer {
            state: LexerState::Resource,
            source,
            ptr: 0,
            entry_start: 0,
            buffer: None,
            peek: None,
            next_ptr: 0,
            done: false,
        }
    }

    fn get_ident(&mut self) -> Token {
        let start = self.ptr;
        self.ptr += 1;
        while let Some(b) = self.source.get(self.ptr) {
            if !b.is_ascii_alphanumeric() && *b != b'-' && *b != b'_' {
                break;
            }
            self.ptr += 1;
        }
        Token::Identifier(start..self.ptr)
    }

    fn tokenize_resource(&mut self, b: u8) -> LexerResult {
        match b {
            b if b.is_ascii_alphabetic() => {
                self.entry_start = self.ptr;
                self.state = LexerState::Message;
                Ok(self.get_ident())
            }
            b'-' => {
                self.ptr += 1;
                Ok(Token::MinusSign)
            }
            b'#' => {
                let start = self.ptr;
                self.ptr += 1;
                while let Some(b'#') = self.source.get(self.ptr) {
                    self.ptr += 1;
                }
                self.state = LexerState::Comment;
                Ok(match self.ptr - start {
                    1 => Token::CommentSign,
                    2 => Token::GroupCommentSign,
                    3 => Token::ResourceCommentSign,
                    _ => panic!(),
                })
            }
            b'\n' => {
                self.ptr += 1;
                Ok(Token::Eol(self.ptr - 1))
            }
            _ => {
                self.entry_start = self.ptr;
                Err(LexerError::Unknown)
            }
        }
    }

    fn tokenize_message(&mut self, b: u8) -> LexerResult {
        if b == b' ' {
            self.ptr += 1;
        }

        while let Some(b) = self.source.get(self.ptr) {
            if *b == b' ' {
                self.ptr += 1;
                continue;
            }

            match self.source.get(self.ptr) {
                Some(b'=') => {
                    self.state = LexerState::Pattern;
                    self.ptr += 1;
                    return Ok(Token::EqSign);
                }
                Some(b'a'..=b'z') => {
                    return Ok(self.get_ident());
                }
                None => return Err(LexerError::Unknown),
                _ => {
                    // println!("{:#}", self.source[self.ptr]);
                    return Err(LexerError::Unknown);
                }
            }
        }
        return Err(LexerError::Unknown);
    }

    fn tokenize_pattern(&mut self, b: u8) -> LexerResult {
        let indent_start = self.ptr;
        let mut in_indent = true;
        let mut start = self.ptr;
        let mut indent = 0;

        if b == b' ' {
            self.ptr += 1;
        }

        while let Some(b) = self.source.get(self.ptr) {
            if in_indent {
                if *b == b' ' {
                    self.ptr += 1;
                    continue;
                } else {
                    start = self.ptr;
                    indent = start - indent_start;
                    in_indent = false;
                }
            }
            match b {
                b'{' => {
                    self.state = LexerState::Expression;
                    return Ok(Token::OpenCurlyBraces);
                }
                b'\n' if start == self.ptr => {
                    self.ptr += 1;
                    match self.check_next_line() {
                        NextLine::TextContinuation => return Ok(Token::Eol(self.ptr - 1)),
                        NextLine::Attribute => {
                            self.state = LexerState::Message;
                            self.buffer = Some(Token::Dot);
                            self.ptr += 1;
                            return Ok(Token::Eot);
                        }
                        NextLine::NewEntry => {
                            self.state = LexerState::Resource;
                            return Ok(Token::Eot);
                        }
                    }
                }
                b'\n' => {
                    return Ok(Token::Text(indent, start..self.ptr));
                }
                _ => {}
            }
            self.ptr += 1;
        }
        Ok(Token::Text(indent, start..self.ptr))
    }

    fn tokenize_comment(&mut self, b: u8) -> LexerResult {
        if b != b' ' {
            if b != b'\n' {
                panic!();
            }
            self.state = LexerState::Resource;
            self.entry_start = self.ptr;
            self.ptr += 1;
            return Ok(Token::Text(0, self.ptr - 1..self.ptr - 1));
        }

        self.ptr += 1;

        let start = self.ptr;

        let mut end_vector = 0;

        while let Some(b) = self.source.get(self.ptr) {
            self.ptr += 1;
            if *b == b'\n' {
                end_vector = 1;
                break;
            }
        }
        self.state = LexerState::Resource;
        self.entry_start = self.ptr;
        Ok(Token::Text(0, start..(self.ptr - end_vector)))
    }

    fn tokenize_expression(&mut self, b: u8) -> LexerResult {
        let mut b = b;
        loop {
            match b {
                b'}' => {
                    self.ptr += 1;
                    break;
                }
                b'"' => {
                    self.ptr += 1;
                    self.state = LexerState::StringLiteral;
                    return Ok(Token::DoubleQuote);
                }
                b'-' => {
                    self.ptr += 1;
                    return Ok(Token::MinusSign);
                }
                b'.' => {
                    self.ptr += 1;
                    return Ok(Token::Dot);
                }
                b'a'..=b'z' => {
                    return Ok(self.get_ident());
                }
                b if b.is_ascii_digit() => {
                    let start = self.ptr;
                    self.ptr += 1;
                    while let Some(b) = self.source.get(self.ptr) {
                        if !b.is_ascii_digit() {
                            break;
                        }
                        self.ptr += 1;
                    }
                    return Ok(Token::Number(start..self.ptr));
                }
                _ => {}
            }

            self.ptr += 1;
            if let Some(b2) = self.source.get(self.ptr) {
                b = *b2;
            } else {
                //XXX: Error
                break;
            }
        }
        self.state = LexerState::Pattern;
        Ok(Token::CloseCurlyBraces)
    }

    fn tokenize_string_literal(&mut self, b: u8) -> LexerResult {
        let start = self.ptr;
        let mut b = b;
        loop {
            match b {
                b'"' => {
                    self.ptr += 1;
                    self.state = LexerState::Expression;
                    self.buffer = Some(Token::DoubleQuote);
                    return Ok(Token::Text(0, start..self.ptr - 1));
                }
                _ => {}
            }

            self.ptr += 1;
            if let Some(b2) = self.source.get(self.ptr) {
                b = *b2;
            } else {
                panic!();
            }
        }
    }

    fn check_next_line(&mut self) -> NextLine {
        let search_start = self.ptr;
        let mut indent_start = self.ptr;

        while let Some(b) = self.source.get(self.ptr) {
            if *b != b' ' {
                if *b == b'\n' {
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
            self.ptr -= 1;
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

    fn get_token(&mut self) -> LexerOptionResult {
        if self.done {
            return Ok(None);
        }
        if self.buffer.is_some() {
            Ok(self.buffer.take())
        } else if self.peek.is_some() {
            Ok(self.peek.take())
        } else if let Some(b) = self.source.get(self.ptr) {
            let token = match self.state {
                LexerState::Resource => self.tokenize_resource(*b),
                LexerState::Message => self.tokenize_message(*b),
                LexerState::Pattern => self.tokenize_pattern(*b),
                LexerState::Expression => self.tokenize_expression(*b),
                LexerState::Comment => self.tokenize_comment(*b),
                LexerState::StringLiteral => self.tokenize_string_literal(*b),
            };
            match token {
                Ok(token) => Ok(Some(token)),
                Err(err) => Err(err)
            }
        } else {
            self.done = true;
            Ok(None)
        }
    }

    fn collect_junk_range(&mut self) -> Range<usize> {
        if self.done {
            return self.entry_start..self.source.len();
        }
        while let Some(b) = self.source.get(self.ptr) {
            self.ptr += 1;
            if b == &b'\n' {
                if let Some(b) = self.source.get(self.ptr) {
                    if b.is_ascii_alphabetic() || b == &b'#' || b == &b'-' {
                        self.next_ptr = self.ptr;
                        return self.entry_start..self.ptr;
                    }
                }
            }
        }
        self.next_ptr = self.ptr;
        self.entry_start..self.ptr
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if let Some(ref token) = self.peek {
            Some(token)
        } else {
            self.next_ptr = self.ptr;
            self.peek = self.next();
            self.peek.as_ref()
        }
    }

    #[inline]
    fn next(&mut self) -> Option<Token> {
        self.get_token().unwrap_or_else(|_| {
            let junk_range = self.collect_junk_range();
            self.state = LexerState::Resource;
            Some(Token::Junk(junk_range))
        })
    }

    pub fn get_junk(&mut self) -> Range<usize> {
        self.buffer = None;
        self.ptr = self.next_ptr;
        self.peek = None;
        let junk_range = self.collect_junk_range();
        self.state = LexerState::Resource;
        junk_range
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
