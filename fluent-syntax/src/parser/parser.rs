use super::ast;
use super::lexer::Lexer;
use super::lexer::Token;
use std::iter::Peekable;
use std::ops::Range;

pub struct Parser<'p> {
    source: &'p str,
    lexer: Peekable<Lexer<'p>>,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p str) -> Self {
        Parser {
            lexer: Lexer::new(source.as_bytes()).peekable(),
            source,
        }
    }

    pub fn parse(mut self) -> ast::Resource<'p> {
        let mut body = vec![];

        let mut last_comment = None;

        while let Some(token) = self.lexer.next() {
            match token {
                Token::Identifier(r) => {
                    let msg = self.get_message(r, last_comment.take());
                    body.push(ast::ResourceEntry::Entry(ast::Entry::Message(msg)));
                }
                Token::MinusSign => {
                    let term = self.get_term(last_comment.take());
                    body.push(ast::ResourceEntry::Entry(ast::Entry::Term(term)));
                }
                Token::Eol(_) => {
                    if let Some(comment) = last_comment.take() {
                        body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
                    }
                }
                c @ Token::CommentSign
                | c @ Token::GroupCommentSign
                | c @ Token::ResourceCommentSign => {
                    let comment = self.get_comment(&c);
                    if let Some(comment) = last_comment.replace(comment) {
                        body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
                    }
                }
                _ => panic!(),
            }
        }

        if let Some(comment) = last_comment.take() {
            body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
        }

        ast::Resource {
            body: body.into_boxed_slice(),
        }
    }

    fn get_message(
        &mut self,
        id: Range<usize>,
        comment: Option<ast::Comment<'p>>,
    ) -> ast::Message<'p> {
        let id = ast::Identifier {
            name: &self.source[id],
        };

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

    fn get_term(&mut self, comment: Option<ast::Comment<'p>>) -> ast::Term<'p> {
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

    fn maybe_get_pattern(&mut self) -> Option<ast::Pattern<'p>> {
        let mut pe = vec![];
        loop {
            match self.lexer.next() {
                Some(Token::Text(_, r)) => {
                    let te = ast::PatternElement::TextElement(&self.source[r]);
                    pe.push(te);
                }
                Some(Token::Eol(i)) => {
                    if !pe.is_empty() {
                        let te = ast::PatternElement::TextElement(&self.source[i..i + 1]);
                        pe.push(te);
                    }
                }
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

    fn get_pattern(&mut self) -> ast::Pattern<'p> {
        let mut pe = vec![];
        loop {
            match self.lexer.next() {
                Some(Token::Text(_, r)) => {
                    let te = ast::PatternElement::TextElement(&self.source[r]);
                    pe.push(te);
                }
                Some(Token::Eol(i)) => {
                    if !pe.is_empty() {
                        let te = ast::PatternElement::TextElement(&self.source[i..i + 1]);
                        pe.push(te);
                    }
                }
                Some(Token::Eot) | None => {
                    break;
                }
                _ => panic!(),
            }
        }
        ast::Pattern {
            elements: pe.into_boxed_slice(),
        }
    }

    fn get_identifier(&mut self) -> ast::Identifier<'p> {
        match self.lexer.next() {
            Some(Token::Identifier(r)) => ast::Identifier {
                name: &self.source[r],
            },
            _ => panic!(),
        }
    }

    fn get_attribute(&mut self) -> ast::Attribute<'p> {
        self.lexer.next(); // Dot
        let id = self.get_identifier();
        self.lexer.next(); // EqSign
        let value = self.get_pattern();
        ast::Attribute { id, value }
    }

    fn get_comment(&mut self, token: &Token) -> ast::Comment<'p> {
        let mut pe = vec![];
        let comment_type_token = token;
        let comment_type = match comment_type_token {
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
                pe.push(&self.source[r]);
            }
            _ => panic!(),
        }
        while let Some(token) = self.lexer.peek() {
            if token != comment_type_token {
                break;
            }
            self.lexer.next();
            match self.lexer.next() {
                Some(Token::Text(indent, mut r)) => {
                    if indent > 0 {
                        r.start -= indent - 1;
                    }
                    pe.push(&self.source[r]);
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
