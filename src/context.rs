use std::collections::HashMap;

use super::syntax::ast;
use super::syntax::parse;
use super::types;
use super::resolver;

pub struct MessageContext {
    locales: Vec<String>,
    pub messages: HashMap<String, ast::Entry>,
}

impl MessageContext {
    pub fn new(locales: Vec<String>) -> MessageContext {
        MessageContext {
            locales: locales,
            messages: HashMap::new(),
        }
    }

    pub fn has_message(&self, id: &str) -> bool {
        self.messages.contains_key(id)
    }

    pub fn get_message(&self, id: &str) -> Option<&ast::Entry> {
        self.messages.get(id)
    }

    pub fn add_messages(&mut self, source: &str) {
        let res = parse(source).unwrap_or_else(|x| x.0);

        for entry in res.body {
            let id = match entry {
                ast::Entry::Message { ref id, .. } => id.name.clone(),
                _ => continue,
            };

            match entry {
                ast::Entry::Message { .. } => {
                    self.messages.insert(id, entry);
                }
                _ => continue,
            };
        }
    }


    pub fn format(&self,
                  message: &ast::Entry,
                  args: Option<&HashMap<String, String>>)
                  -> Option<String> {
        let result = resolver::resolve(&self, args, message);

        return Some(types::value_of(result));
    }
}
