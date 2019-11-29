use super::ast;
use super::lexer::Lexer;
use super::lexer::Token;
use std::iter::Peekable;
use std::ops::Range;

pub struct Parser<'p> {
    lexer: Peekable<Lexer<'p>>,
    resource: Vec<ast::ResourceEntry>,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p [u8]) -> Self {
        Parser {
            lexer: Lexer::new(source).peekable(),
            resource: vec![],
        }
    }

    pub fn parse(mut self) -> ast::Resource {
        while let Some(entry) = self.get_entry() {
            self.resource.push(ast::ResourceEntry::Entry(entry));
        }

        ast::Resource {
            body: self.resource.into_boxed_slice(),
        }
    }

    fn get_entry(&mut self) -> Option<ast::Entry> {
        let mut last_comment = None;
        while let Some(token) = self.lexer.next() {
            match token {
                Token::Identifier(r) => {
                    let msg = self.get_message(r, last_comment.take());
                    return Some(ast::Entry::Message(msg));
                }
                Token::MinusSign => {
                    let term = self.get_term(last_comment.take());
                    return Some(ast::Entry::Term(term));
                }
                Token::Eol => {
                    if let Some(comment) = last_comment.take() {
                        return Some(ast::Entry::Comment(comment));
                    }
                }
                c @ Token::CommentSign
                | c @ Token::GroupCommentSign
                | c @ Token::ResourceCommentSign => {
                    let comment = self.get_comment(&c);
                    last_comment = Some(comment);
                }
                _ => panic!(),
            }
        }
        if let Some(comment) = last_comment.take() {
            return Some(ast::Entry::Comment(comment));
        }
        None
    }

    fn get_message(&mut self, id: Range<usize>, comment: Option<ast::Comment>) -> ast::Message {
        let id = ast::Identifier { name: id };

        self.lexer.next(); // EqSign

        let pattern = self.maybe_get_pattern();

        let mut attributes = vec![];
        while let Some(Token::Dot) = self.lexer.peek() {
            attributes.push(self.get_attribute());
        }
        ast::Message {
            id,
            value: pattern,
            attributes: attributes.into_boxed_slice(),
            comment,
        }
    }

    fn get_term(&mut self, comment: Option<ast::Comment>) -> ast::Term {
        let id = self.get_identifier();

        self.lexer.next(); // EqSign

        let pattern = self.get_pattern();

        let mut attributes = vec![];
        while let Some(Token::Dot) = self.lexer.peek() {
            attributes.push(self.get_attribute());
        }
        ast::Term {
            id,
            value: pattern,
            attributes: attributes.into_boxed_slice(),
            comment,
        }
    }

    fn maybe_get_pattern(&mut self) -> Option<ast::Pattern> {
        let mut pe = vec![];
        loop {
            match self.lexer.next() {
                Some(Token::Text(_, r)) => {
                    let te = ast::PatternElement::TextElement(r);
                    pe.push(te);
                }
                Some(Token::Eol) => {}
                Some(Token::Eot) => {
                    break;
                }
                None => {
                    break;
                }
                b => {
                    println!("{:#?}", b);
                    panic!();
                }
            }
        }
        if pe.is_empty() {
            None
        } else {
            Some(ast::Pattern {
                elements: pe.into_boxed_slice(),
            })
        }
    }

    fn get_pattern(&mut self) -> ast::Pattern {
        let mut pe = vec![];
        loop {
            match self.lexer.next() {
                Some(Token::Text(_, r)) => {
                    let te = ast::PatternElement::TextElement(r);
                    pe.push(te);
                }
                Some(Token::Eot) => {
                    break;
                }
                None => {
                    break;
                }
                _ => panic!(),
            }
        }
        ast::Pattern {
            elements: pe.into_boxed_slice(),
        }
    }

    fn get_identifier(&mut self) -> ast::Identifier {
        match self.lexer.next() {
            Some(Token::Identifier(r)) => ast::Identifier { name: r },
            _ => panic!(),
        }
    }

    fn get_attribute(&mut self) -> ast::Attribute {
        self.lexer.next(); // Dot
        let id = self.get_identifier();
        self.lexer.next(); // EqSign
        let value = self.get_pattern();
        ast::Attribute { id, value }
    }

    fn get_comment(&mut self, token: &Token) -> ast::Comment {
        let mut pe = vec![];
        let comment_type = match token {
            Token::CommentSign => ast::CommentType::Regular,
            Token::GroupCommentSign => ast::CommentType::Group,
            Token::ResourceCommentSign => ast::CommentType::Resource,
            _ => panic!(),
        };
        match self.lexer.next() {
            Some(Token::Text(indent, mut r)) => {
                if indent > 0 {
                    r.start -= indent - 1;
                }
                pe.push(r);
            }
            _ => panic!(),
        }
        while let Some(Token::CommentSign) = self.lexer.peek() {
            self.lexer.next();
            match self.lexer.next() {
                Some(Token::Text(indent, mut r)) => {
                    if indent > 0 {
                        r.start -= indent - 1;
                    }
                    pe.push(r);
                }
                _ => panic!(),
            }
        }
        let content = pe.into_boxed_slice();
        ast::Comment {
            comment_type,
            content,
        }
    }
}
