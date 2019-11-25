use super::ast;
use super::lexer::Lexer;
use super::lexer::Token;
use std::ops::Range;

pub struct Parser<'p> {
    lexer: Lexer<'p>,
    resource: Vec<ast::ResourceEntry>,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p [u8]) -> Self {
        Parser {
            lexer: Lexer::new(source),
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
        self.lexer.next().map(|token| match token {
            Token::Identifier(r) => {
                let msg = self.get_message(r);
                ast::Entry::Message(msg)
            }
            Token::CommentSign => match self.lexer.next() {
                Some(Token::Text(r)) => {
                    let content = vec![r];
                    let comment = ast::Comment {
                        comment_type: ast::CommentType::Regular,
                        content: content.into_boxed_slice(),
                    };
                    ast::Entry::Comment(comment)
                }
                _ => panic!(),
            },
            _ => panic!(),
        })
    }

    fn get_message(&mut self, id: Range<usize>) -> ast::Message {
        let id = ast::Identifier { name: id };

        self.lexer.next(); // EqSign

        match self.lexer.next() {
            Some(Token::Text(r)) => ast::Message { id, value: Some(r) },
            _ => panic!(),
        }
    }
}
