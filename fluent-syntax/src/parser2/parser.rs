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
                Token::CommentSign => {
                    let comment = self.get_comment();
                    last_comment = Some(comment);
                }
                _ => panic!(),
            }
        }
        None
    }

    fn get_message(&mut self, id: Range<usize>, comment: Option<ast::Comment>) -> ast::Message {
        let id = ast::Identifier { name: id };

        self.lexer.next(); // EqSign

        let pattern = self.maybe_get_pattern();

        let mut attributes = vec![];
        while let Some(Token::Dot) = self.lexer.peek() {
            self.lexer.next(); // Dot
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
            self.lexer.next(); // Dot
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
        match self.lexer.next() {
            Some(Token::Text(_, r)) => {
                let te = ast::PatternElement::TextElement(r);
                Some(ast::Pattern {
                    elements: Box::new([te]),
                })
            }
            Some(Token::Eol) => None,
            b @ _ => {
                println!("{:#?}", b);
                panic!()
            }
        }
    }

    fn get_pattern(&mut self) -> ast::Pattern {
        match self.lexer.next() {
            Some(Token::Text(_, r)) => {
                let te = ast::PatternElement::TextElement(r);
                ast::Pattern {
                    elements: Box::new([te]),
                }
            }
            _ => panic!(),
        }
    }

    fn get_identifier(&mut self) -> ast::Identifier {
        match self.lexer.next() {
            Some(Token::Identifier(r)) => ast::Identifier { name: r },
            _ => panic!(),
        }
    }

    fn get_attribute(&mut self) -> ast::Attribute {
        self.lexer.next();
        self.lexer.next();
        self.lexer.next();
        ast::Attribute {}
    }

    fn get_comment(&mut self) -> ast::Comment {
        match self.lexer.next() {
            Some(Token::Text(indent, mut r)) => {
                if indent > 0 {
                    r.start -= indent - 1;
                }
                let content = Box::new([r]);
                ast::Comment {
                    comment_type: ast::CommentType::Regular,
                    content,
                }
            }
            _ => panic!(),
        }
    }
}
