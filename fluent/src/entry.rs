//! `Entry` is used to store Messages, Terms and Functions in `MessageContext` instances.
use std::collections::hash_map::HashMap;

use super::types::FluentValue;
use fluent_syntax::ast;

type FluentFunction<'ctx> =
    Box<'ctx + Fn(&[Option<FluentValue>], &HashMap<String, FluentValue>) -> Option<FluentValue>>;

pub enum Entry<'ctx> {
    Message(ast::Message),
    Term(ast::Term),
    Function(FluentFunction<'ctx>),
}

pub trait GetEntry<'ctx> {
    fn get_term(&self, id: &str) -> Option<&ast::Term>;
    fn get_message(&self, id: &str) -> Option<&ast::Message>;
    fn get_function(&self, id: &str) -> Option<&FluentFunction<'ctx>>;
}

impl<'ctx> GetEntry<'ctx> for HashMap<String, Entry<'ctx>> {
    fn get_term(&self, id: &str) -> Option<&ast::Term> {
        self.get(id).and_then(|entry| match entry {
            Entry::Term(term) => Some(term),
            _ => None,
        })
    }

    fn get_message(&self, id: &str) -> Option<&ast::Message> {
        self.get(id).and_then(|entry| match entry {
            Entry::Message(message) => Some(message),
            _ => None,
        })
    }

    fn get_function(&self, id: &str) -> Option<&FluentFunction<'ctx>> {
        self.get(id).and_then(|entry| match entry {
            Entry::Function(function) => Some(function),
            _ => None,
        })
    }
}
