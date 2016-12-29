use std::collections::HashMap;

use super::syntax::runtime::parse;
use super::syntax::runtime::parser::ParserError;
use self::resolver::resolve;

use super::syntax::runtime::ast;

#[derive(Debug)]
pub enum ContextError {
    Generic,
}

pub struct MessageContext {
    entries: HashMap<String, ast::Entry>,
}

impl MessageContext {
    pub fn new() -> MessageContext {
        MessageContext { entries: HashMap::new() }
    }

    pub fn add_messages(&mut self, source: &str) -> Result<(), ParserError> {
        let res = parse(source)?;

        for entry in res.0 {
            match entry {
                ast::Entry::Message { id, value, traits } => {
                    self.entries.insert(id.clone(),
                                        ast::Entry::Message {
                                            id: id,
                                            value: value,
                                            traits: traits,
                                        });
                }
            }
        }

        Ok(())
    }

    pub fn get_message(&self, id: &str) -> Option<&ast::Value> {
        self.entries.get(id)
    }

    pub fn format(&self, value: &ast::Value) -> Result<String, ContextError> {
        match resolve(self, value) {
            Ok(msg) => Ok(msg),
            Err(_) => Err(ContextError::Generic),
        }
    }
}

pub mod resolver;
