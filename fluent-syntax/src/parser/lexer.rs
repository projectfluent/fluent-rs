use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(Range<usize>),
    EqSign,
    CommentSign,
    GroupCommentSign,
    ResourceCommentSign,
    Eol(usize), // one or more empty lines
    Eot,        // End Of Text
    Dot,
    MinusSign,
    Indent(usize),
    Text(Range<usize>),
    Junk(Range<usize>),
    OpenCurlyBraces,
    CloseCurlyBraces,
    DoubleQuote,
    Number(Range<usize>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LexerState {
    Resource,
    Pattern,
    PatternContinuation,
    Message,
    Comment,
    Expression,
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    Unknown,
}

type LexerResult = Result<Token, LexerError>;
type LexerOptionResult = Result<Option<Token>, LexerError>;

#[derive(Clone, Debug)]
pub struct Lexer<'l> {
    state: LexerState,
    source: &'l [u8],
    ptr: usize,
    entry_start: usize,
    peeked: Option<Option<Token>>,
}

#[derive(Debug, PartialEq)]
pub enum NextLine {
    TextContinuation(usize),
    Attribute,
    NewEntry,
    Eol(usize),
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l [u8]) -> Self {
        Lexer {
            state: LexerState::Resource,
            source,
            ptr: 0,
            entry_start: 0,
            peeked: None,
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

    fn tokenize_message(&mut self) -> LexerOptionResult {
        match self.first_after_inline() {
            Some(b'=') => {
                self.ptr += 1;
                self.state = LexerState::Pattern;
                Ok(Some(Token::EqSign))
            }
            Some(b'.') => {
                self.ptr += 1;
                Ok(Some(Token::Dot))
            }
            Some(b'a'..=b'z') | Some(b'A'..=b'Z') => {
                let ident = self.get_ident();
                Ok(Some(ident))
            }
            _ => Err(LexerError::Unknown),
        }
    }

    fn tokenize_resource(&mut self) -> LexerOptionResult {
        self.entry_start = self.ptr;
        if let Some(b) = self.source.get(self.ptr) {
            match b {
                b'\n' => {
                    self.ptr += 1;
                    Ok(Some(Token::Eol(1)))
                }
                b'a'..=b'z' | b'A'..=b'Z' => {
                    let ident = self.get_ident();
                    self.state = LexerState::Message;
                    Ok(Some(ident))
                }
                b'#' => {
                    let sigil;
                    self.ptr += 1;
                    if let Some(b'#') = self.source.get(self.ptr) {
                        self.ptr += 1;
                        if let Some(b'#') = self.source.get(self.ptr) {
                            self.ptr += 1;
                            sigil = Token::ResourceCommentSign;
                        } else {
                            sigil = Token::GroupCommentSign;
                        }
                    } else {
                        sigil = Token::CommentSign;
                    }
                    match self.source.get(self.ptr) {
                        Some(b' ') => {
                            self.ptr += 1;
                            self.state = LexerState::Comment;
                            Ok(Some(sigil))
                        }
                        Some(b'\n') => Ok(Some(sigil)),
                        _ => Err(LexerError::Unknown),
                    }
                }
                b'-' => {
                    self.ptr += 1;
                    Ok(Some(Token::MinusSign))
                }
                _ => Err(LexerError::Unknown),
            }
        } else {
            Ok(None)
        }
    }

    fn skip_inline_ws(&mut self) {
        while let Some(b' ') = self.source.get(self.ptr) {
            self.ptr += 1;
        }
    }

    fn first_after_inline(&mut self) -> Option<&u8> {
        loop {
            match self.source.get(self.ptr) {
                Some(b' ') => {
                    self.ptr += 1;
                }
                b => {
                    return b;
                }
            }
        }
    }

    fn tokenize_pattern(&mut self) -> LexerOptionResult {
        let mut b = self.source.get(self.ptr);

        loop {
            match b {
                Some(b' ') => {
                    self.ptr += 1;
                    b = self.source.get(self.ptr);
                }
                Some(b'\n') => {
                    let mut eol = self.ptr;
                    loop {
                        self.ptr += 1;
                        match self.first_after_inline() {
                            Some(b'\n') => {
                                eol = self.ptr;
                            }
                            Some(b'.') => {
                                self.state = LexerState::Message;
                                return Ok(Some(Token::Eot));
                            }
                            None => {
                                self.state = LexerState::Message;
                                return Ok(Some(Token::Eot));
                            }
                            _ => {
                                let indent = self.ptr - (eol + 1);
                                self.state = LexerState::PatternContinuation;
                                return Ok(Some(Token::Indent(indent)));
                            }
                        }
                    }
                }
                _ => break,
            }
        }

        let next_token;
        let start = self.ptr;
        let mut end;

        loop {
            match b {
                Some(b'\n') => {
                    end = self.ptr;
                    match self.next_lines_info() {
                        NextLine::NewEntry => {
                            self.state = LexerState::Resource;
                            next_token = Token::Eot;
                        }
                        NextLine::Eol(eols) => {
                            self.state = LexerState::PatternContinuation;
                            next_token = Token::Eol(eols);
                        }
                        NextLine::Attribute => {
                            self.state = LexerState::Message;
                            next_token = Token::Eot;
                        }
                        NextLine::TextContinuation(r) => {
                            self.state = LexerState::PatternContinuation;
                            end += 1;
                            next_token = Token::Indent(r);
                        }
                    }
                    break;
                }
                Some(b'{') => {
                    self.state = LexerState::Expression;
                    end = self.ptr;
                    self.ptr += 1;
                    self.skip_inline_ws();
                    next_token = Token::OpenCurlyBraces;
                    break;
                }
                None => {
                    end = self.ptr;
                    self.state = LexerState::Resource;
                    next_token = Token::Eot;
                    break;
                }
                _ => {
                    self.ptr += 1;
                    b = self.source.get(self.ptr);
                }
            }
        }
        if self.state != LexerState::Expression {
            while let Some(b' ') = self.source.get(end - 1) {
                end -= 1;
            }
        }
        if start != end {
            self.peeked = Some(Some(next_token));
            Ok(Some(Token::Text(start..end)))
        } else {
            Ok(Some(next_token))
        }
    }

    fn tokenize_pattern_continuation(&mut self) -> LexerOptionResult {
        let start = self.ptr;
        let mut end;
        let next_token;

        loop {
            match self.source.get(self.ptr) {
                Some(b'\n') => {
                    end = self.ptr;
                    match self.next_lines_info() {
                        NextLine::NewEntry => {
                            self.state = LexerState::Resource;
                            next_token = Token::Eot;
                        }
                        NextLine::Eol(eols) => {
                            next_token = Token::Eol(eols);
                        }
                        NextLine::Attribute => {
                            self.state = LexerState::Message;
                            next_token = Token::Eot;
                        }
                        NextLine::TextContinuation(r) => {
                            end += 1;
                            next_token = Token::Indent(r);
                        }
                    }
                    break;
                }
                Some(b'{') => {
                    self.state = LexerState::Expression;
                    end = self.ptr;
                    self.ptr += 1;
                    self.skip_inline_ws();
                    next_token = Token::OpenCurlyBraces;
                    break;
                }
                None => {
                    self.state = LexerState::Resource;
                    end = self.ptr;
                    self.ptr += 1;
                    next_token = Token::Eot;
                    break;
                }
                _ => {}
            }
            self.ptr += 1;
        }
        if self.state != LexerState::Expression {
            while let Some(b' ') = self.source.get(end - 1) {
                end -= 1;
            }
        }
        if start != end {
            self.peeked = Some(Some(next_token));
            Ok(Some(Token::Text(start..end)))
        } else {
            Ok(Some(next_token))
        }
    }

    fn next_lines_info(&mut self) -> NextLine {
        let mut last_eol = self.ptr;
        let mut eols = 0;
        self.ptr += 1;
        loop {
            let line_start = self.ptr;
            match self.first_after_inline() {
                Some(b'.') => return NextLine::Attribute,
                Some(b'\n') => {
                    last_eol = self.ptr;
                    eols += 1;
                    self.ptr += 1;
                }
                Some(b'{') => {
                    if eols > 0 {
                        self.ptr = last_eol;
                        return NextLine::Eol(eols);
                    } else {
                        return NextLine::TextContinuation(self.ptr - line_start);
                    }
                }
                _ => {
                    let line_indent = self.ptr - line_start;
                    if line_indent > 0 {
                        if eols > 0 {
                            self.ptr = last_eol;
                            return NextLine::Eol(eols);
                        } else {
                            return NextLine::TextContinuation(line_indent);
                        }
                    } else {
                        return NextLine::NewEntry;
                    }
                }
            }
        }
    }

    fn tokenize_expression(&mut self) -> LexerResult {
        match self.source.get(self.ptr) {
            Some(b'0'..=b'9') => {
                let start = self.ptr;
                self.ptr += 1;
                while let Some(b'0'..=b'9') = self.source.get(self.ptr) {
                    self.ptr += 1;
                }
                Ok(Token::Number(start..self.ptr))
            }
            Some(b'a'..=b'z') | Some(b'A'..=b'Z') => {
                let ident = self.get_ident();
                Ok(ident)
            }
            Some(b'.') => {
                self.ptr += 1;
                Ok(Token::Dot)
            }
            Some(b'-') => {
                self.ptr += 1;
                Ok(Token::MinusSign)
            }
            Some(b'"') => {
                self.ptr += 1;
                let start = self.ptr;
                while let Some(b) = self.source.get(self.ptr) {
                    self.ptr += 1;
                    if b == &b'"' {
                        return Ok(Token::Text(start..self.ptr - 1));
                    }
                }
                Err(LexerError::Unknown)
            }
            Some(b' ') => {
                if let Some(b'}') = self.first_after_inline() {
                    self.ptr += 1;
                    self.state = LexerState::PatternContinuation;
                    Ok(Token::CloseCurlyBraces)
                } else {
                    Err(LexerError::Unknown)
                }
            }
            Some(b'}') => {
                self.ptr += 1;
                self.state = LexerState::PatternContinuation;
                Ok(Token::CloseCurlyBraces)
            }
            _ => Err(LexerError::Unknown),
        }
    }

    fn tokenize_comment(&mut self) -> LexerResult {
        let start = self.ptr;
        while let Some(b) = self.source.get(self.ptr) {
            self.ptr += 1;
            if &b'\n' == b {
                break;
            }
        }
        self.state = LexerState::Resource;
        Ok(Token::Text(start..self.ptr - 1))
    }

    #[inline]
    fn get_token(&mut self) -> LexerOptionResult {
        match self.state {
            LexerState::Resource => self.tokenize_resource(),
            LexerState::PatternContinuation => self.tokenize_pattern_continuation(),
            LexerState::Pattern => self.tokenize_pattern(),
            LexerState::Message => self.tokenize_message(),
            LexerState::Comment => self.tokenize_comment().map(Option::Some),
            LexerState::Expression => self.tokenize_expression().map(Option::Some),
        }
    }

    fn collect_junk_range(&mut self) -> Range<usize> {
        while let Some(b) = self.source.get(self.ptr) {
            if (b.is_ascii_alphabetic() || b == &b'#')
                && self.ptr > 1
                && self.source[self.ptr - 1] == b'\n'
            {
                break;
            }
            self.ptr += 1;
        }
        self.state = LexerState::Resource;
        self.entry_start..self.ptr
    }

    #[inline]
    fn next(&mut self) -> Option<Token> {
        if let Some(token) = self.peeked.take() {
            token
        } else {
            self.get_token().unwrap_or_else(|_| {
                let junk_range = self.collect_junk_range();
                Some(Token::Junk(junk_range))
            })
        }
    }

    #[inline]
    pub fn try_next(&mut self) -> LexerOptionResult {
        if let Some(token) = self.peeked.take() {
            Ok(token)
        } else {
            self.get_token()
        }
    }

    #[inline]
    pub fn expect(&mut self, token: Token) -> Result<(), LexerError> {
        if self.try_next()? != Some(token) {
            Err(LexerError::Unknown)
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn try_peek(&mut self) -> Result<Option<&Token>, LexerError> {
        if let Some(ref token) = self.peeked {
            Ok(token.as_ref())
        } else {
            self.get_token().map(move |token| {
                self.peeked = Some(token);
                self.peeked.as_ref().unwrap().as_ref()
            })
        }
    }

    #[inline]
    pub fn take_if(&mut self, token: Token) -> Result<bool, LexerError> {
        if let Some(ref t) = self.peeked {
            if &Some(token) == t {
                self.peeked = None;
                Ok(true)
            } else {
                Ok(false)
            }
        } else if let Some(t) = self.get_token()? {
            if t == token {
                Ok(true)
            } else {
                self.peeked = Some(Some(t));
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    pub fn get_junk(&mut self) -> Range<usize> {
        self.collect_junk_range()
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
