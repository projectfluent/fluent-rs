use std::collections::HashMap;

use super::syntax::ast;
use super::syntax::parse;
use super::types::FluentValue;
use super::resolve::{Env, ResolveValue};


#[allow(dead_code)]
pub struct MessageContext {
    locales: Vec<String>,
    messages: HashMap<String, ast::Message>,
}

pub trait VecOrOne<String> {
    fn into_vec(self) -> Vec<String>;
}

impl VecOrOne<String> for Vec<String> {
    fn into_vec(self) -> Vec<String> {
        self
    }
}

impl VecOrOne<String> for String {
    fn into_vec(self) -> Vec<String> {
        vec![self]
    }
}

impl VecOrOne<String> for &'static str {
    fn into_vec(self) -> Vec<String> {
        vec![String::from(self)]
    }
}

impl VecOrOne<String> for Vec<&'static str> {
    fn into_vec(self) -> Vec<String> {
        self.iter().map(|&loc| String::from(loc)).collect()
    }
}

impl MessageContext {
    pub fn new<L>(locales: L) -> MessageContext
    where
        L: VecOrOne<String>,
    {
        MessageContext {
            locales: locales.into_vec(),
            messages: HashMap::new(),
        }
    }

    pub fn has_message(&self, id: &str) -> bool {
        self.messages.contains_key(id)
    }

    pub fn get_message(&self, id: &str) -> Option<&ast::Message> {
        self.messages.get(id)
    }

    pub fn add_messages(&mut self, source: &str) {
        let res = parse(source).unwrap_or_else(|x| x.0);

        for entry in res.body {
            let id = match entry {
                ast::Entry::Message(ast::Message { ref id, .. }) => id.name.clone(),
                _ => continue,
            };

            match entry {
                ast::Entry::Message(message) => {
                    self.messages.insert(id, message);
                }
                _ => continue,
            };
        }
    }


    pub fn format<T: ResolveValue>(
        &self,
        resolvable: &T,
        args: Option<&HashMap<&str, FluentValue>>,
    ) -> Option<String> {
        let env = Env { ctx: self, args };
        resolvable.to_value(&env).map(|value| value.format())
    }
}
