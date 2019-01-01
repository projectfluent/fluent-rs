//! `Entry` is used to store Messages, Terms and Functions in `FluentBundle` instances.
use std::collections::hash_map::HashMap;

use super::types::FluentValue;
use fluent_syntax::ast;

type FluentFunction<'bundle> = Box<
    'bundle
        + Fn(&[Option<FluentValue>], &HashMap<String, FluentValue>) -> Option<FluentValue>
        + Send
        + Sync,
>;

pub enum Entry<'bundle> {
    Message(&'bundle ast::Message<'bundle>),
    Term(&'bundle ast::Term<'bundle>),
    Function(FluentFunction<'bundle>),
}

pub trait GetEntry<'bundle> {
    fn get_term(&self, id: &str) -> Option<&ast::Term>;
    fn get_message(&self, id: &str) -> Option<&ast::Message>;
    fn get_function(&self, id: &str) -> Option<&FluentFunction<'bundle>>;
}

impl<'bundle> GetEntry<'bundle> for HashMap<String, Entry<'bundle>> {
    fn get_term(&self, id: &str) -> Option<&ast::Term> {
        self.get(id).and_then(|entry| match *entry {
            Entry::Term(term) => Some(term),
            _ => None,
        })
    }

    fn get_message(&self, id: &str) -> Option<&ast::Message> {
        self.get(id).and_then(|entry| match *entry {
            Entry::Message(message) => Some(message),
            _ => None,
        })
    }

    fn get_function(&self, id: &str) -> Option<&FluentFunction<'bundle>> {
        self.get(id).and_then(|entry| match entry {
            Entry::Function(function) => Some(function),
            _ => None,
        })
    }
}
