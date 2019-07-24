//! `Entry` is used to store Messages, Terms and Functions in `FluentBundle` instances.

use std::borrow::Borrow;
use std::collections::HashMap;

use fluent_syntax::ast;

use crate::bundle::FluentBundle;
use crate::resource::FluentResource;
use crate::types::FluentValue;

pub type FluentFunction = Box<
    dyn for<'a> Fn(&[FluentValue<'a>], &HashMap<String, FluentValue<'a>>) -> FluentValue<'a>
        + Send
        + Sync,
>;

pub enum Entry {
    Message([usize; 2]),
    Term([usize; 2]),
    Function(FluentFunction),
}

pub trait GetEntry {
    fn get_entry_message(&self, id: &str) -> Option<&ast::Message>;
    fn get_entry_term(&self, id: &str) -> Option<&ast::Term>;
    fn get_entry_function(&self, id: &str) -> Option<&FluentFunction>;
}

impl<'bundle, R: Borrow<FluentResource>> GetEntry for FluentBundle<R> {
    fn get_entry_message(&self, id: &str) -> Option<&ast::Message> {
        self.entries.get(id).and_then(|entry| match *entry {
            Entry::Message(pos) => {
                let res = self.resources.get(pos[0])?.borrow();
                if let Some(ast::ResourceEntry::Entry(ast::Entry::Message(ref msg))) =
                    res.ast().body.get(pos[1])
                {
                    return Some(msg);
                } else {
                    return None;
                }
            }
            _ => None,
        })
    }

    fn get_entry_term(&self, id: &str) -> Option<&ast::Term> {
        self.entries.get(id).and_then(|entry| match *entry {
            Entry::Term(pos) => {
                let res = self.resources.get(pos[0])?.borrow();
                if let Some(ast::ResourceEntry::Entry(ast::Entry::Term(ref msg))) =
                    res.ast().body.get(pos[1])
                {
                    return Some(msg);
                } else {
                    return None;
                }
            }
            _ => None,
        })
    }

    fn get_entry_function(&self, id: &str) -> Option<&FluentFunction> {
        self.entries.get(id).and_then(|entry| match entry {
            Entry::Function(function) => Some(function),
            _ => None,
        })
    }
}
