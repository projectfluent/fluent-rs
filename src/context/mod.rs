use std::collections::HashMap;

use super::syntax::parse;
use super::syntax::parser::ParserError;
use self::resolver::resolve;

use super::syntax::ast;

#[derive(Debug)]
pub enum ContextError {
    Generic,
}

pub struct MessageContext {
    messages: HashMap<String, ast::Message>,
}

impl MessageContext {
    pub fn new() -> MessageContext {
        MessageContext { messages: HashMap::new() }
    }

    pub fn add_messages(&mut self, source: &str) -> Result<(), ParserError> {
        let res = parse(source)?;

        for entry in res.body {
            match entry {
                ast::Entry::Message(msg @ ast::Message { .. }) => {
                    self.messages.insert(msg.id.name.clone(), msg);
                }
            }
        }

        Ok(())
    }

    pub fn get_message(&self, id: &str) -> Option<&ast::Message> {
        self.messages.get(id)
    }

    pub fn format(&self, value: &ast::Message) -> Result<String, ContextError> {
        match resolve(self, value) {
            Ok(msg) => Ok(msg),
            Err(_) => Err(ContextError::Generic),
        }
    }
}

pub mod resolver;
