use std::collections::HashMap;

use super::syntax::ast;
use super::syntax::parse;
use super::types;
use super::resolver;

pub enum FluentArgument {
    String(String),
    Number(i8),
}

impl From<String> for FluentArgument {
    fn from(s: String) -> Self {
        return FluentArgument::String(s);
    }
}

impl From<&'static str> for FluentArgument {
    fn from(s: &'static str) -> Self {
        return FluentArgument::String(String::from(s));
    }
}

impl From<i8> for FluentArgument {
    fn from(n: i8) -> Self {
        return FluentArgument::Number(n);
    }
}

#[allow(dead_code)]
pub struct MessageContext {
    locales: Vec<String>,
    messages: HashMap<String, ast::Entry>,
}

pub trait VecOrOne<String> {
  fn into_vec(self) -> Vec<String>;
}

impl VecOrOne<String> for Vec<String> {
    fn into_vec(self) -> Vec<String> {
        return self;
    }
}

impl VecOrOne<String> for String {
    fn into_vec(self) -> Vec<String> {
        return vec![self];
    }
}

impl VecOrOne<String> for &'static str {
    fn into_vec(self) -> Vec<String> {
        return vec![String::from(self)];
    }
}

impl VecOrOne<String> for Vec<&'static str> {
    fn into_vec(self) -> Vec<String> {
        return self.iter().map(|&loc| String::from(loc)).collect();
    }
}

impl MessageContext {
    pub fn new<L>(locales: L) -> MessageContext
      where L: VecOrOne<String> {
        MessageContext {
            locales: locales.into_vec(),
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
                  args: Option<&HashMap<&str, FluentArgument>>)
                  -> Option<String> {
        let result = resolver::resolve(&self, args, message);

        return Some(types::value_of(result));
    }
}
