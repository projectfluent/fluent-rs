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

        match self.lexer.next() {
            Some(Token::Text(_, r)) => {
                let attributes = Box::new([]);

                let te = ast::PatternElement::TextElement(r);
                let pattern = ast::Pattern {
                    elements: Box::new([te]),
                };
                ast::Message {
                    id,
                    value: Some(pattern),
                    attributes,
                    comment,
                }
            }
            _ => panic!(),
        }
    }

    fn get_comment(&mut self) -> ast::Comment {
        match self.lexer.next() {
            Some(Token::Text(indent, mut r)) => {
                r.start -= indent - 1;
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
