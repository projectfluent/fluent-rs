//! `Entry` is used to store Messages, Terms and Functions in `FluentBundle` instances.
use std::collections::hash_map::HashMap;

use super::types::*;
use super::FluentResource;
use std::rc::Rc;
use fluent_syntax::ast;

type FluentFunction<'bundle> = Box<dyn
    'bundle
        + for<'a> Fn(&[FluentValue<'a>], &HashMap<&str, FluentValue<'a>>) -> FluentValue<'a>
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

pub trait GetEntry2 {
    fn get_message(&self, id: &str) -> Option<&ast::Message>;
    fn get_term(&self, id: &str) -> Option<&ast::Term>;
}

impl GetEntry2 for Vec<Rc<FluentResource>> {
    fn get_message(&self, id: &str) -> Option<&ast::Message> {
        for res in self {
            for entry in &res.ast().body {
                let (msg, entry_id) = match entry {
                    ast::ResourceEntry::Entry(ast::Entry::Message(msg @ ast::Message { .. })) => (msg, msg.id.name),
                    _ => continue,
                };
                if id == entry_id {
                    return Some(msg);
                }
            }
        }
        return None;
    }

    fn get_term(&self, id: &str) -> Option<&ast::Term> {
        for res in self {
            for entry in &res.ast().body {
                let (term, entry_id) = match entry {
                    ast::ResourceEntry::Entry(ast::Entry::Term(term @ ast::Term { .. })) => (term, term.id.name),
                    _ => continue,
                };

                if id == entry_id {
                    return Some(term);
                }
            }
        }
        return None;
    }
}