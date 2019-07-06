//! `Entry` is used to store Messages, Terms and Functions in `FluentBundle` instances.

use super::FluentBundle;
use super::bundle::FluentFunction;
use fluent_syntax::ast;

pub trait GetEntry {
    fn get_message(&self, id: &str) -> Option<&ast::Message>;
    fn get_term(&self, id: &str) -> Option<&ast::Term>;
    fn get_function(&self, id: &str) -> Option<&FluentFunction>;
}

impl<'bundle> GetEntry for FluentBundle<'bundle> {
    fn get_message(&self, id: &str) -> Option<&ast::Message> {
        if let Some(pos) = self.entries.get(id) {
            let res = self.resources.get(pos[0]).unwrap();
            if let Some(ast::ResourceEntry::Entry(ast::Entry::Message(ref msg))) = res.ast().body.get(pos[1]) {
                return Some(msg);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn get_term(&self, id: &str) -> Option<&ast::Term> {
        if let Some(pos) = self.entries.get(id) {
            let res = self.resources.get(pos[0]).unwrap();
            if let Some(ast::ResourceEntry::Entry(ast::Entry::Term(ref term))) = res.ast().body.get(pos[1]) {
                return Some(term);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn get_function(&self, id: &str) -> Option<&FluentFunction> {
        self.functions.get(id)
    }
}