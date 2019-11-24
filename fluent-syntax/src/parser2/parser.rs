use super::ast;
use super::lexer::Lexer;
use super::lexer::Token;

pub struct Parser<'p> {
    lexer: Lexer<'p>,
    resource: Vec<ast::ResourceEntry>,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p [u8]) -> Self {
        Parser {
            lexer: Lexer::new(source),
            resource: vec![]
        }
    }

    pub fn parse(mut self) -> ast::Resource {
        let mut id = None;
        while let Some(token) = self.lexer.next() {
            match token {
                Token::Identifier(r) => {
                    id = Some(r);
                },
                Token::Text(r) => {
                    let msg = ast::Message {
                        id: ast::Identifier { name: id.take().unwrap() },
                        value: Some(r),
                    };
                    let entry = ast::Entry::Message(msg);
                    self.resource.push(ast::ResourceEntry::Entry(entry));
                },
                Token::EqSign => {},
            }
        }

        ast::Resource { body: self.resource.into_boxed_slice() }
    }
}
