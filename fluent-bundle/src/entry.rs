//! `Entry` is used to store the lookup information for Messages, Terms and Functions in
//! `FluentBundle` instances.

use std::borrow::Borrow;

use fluent_syntax::ast;

use crate::args::FluentArgs;
use crate::bundle::FluentBundle;
use crate::resource::FluentResource;
use crate::types::FluentValue;
use crate::FluentMessage;

type ResourceIdx = usize;
type EntryIdx = usize;

/// The [`Entry`] stores indexes into the [`FluentBundle`]'s resources for Messages and Terms,
/// and owns the [`Box`] pointers to the [`FluentFunction`].
pub enum Entry {
    Message((ResourceIdx, EntryIdx)),
    Term((ResourceIdx, EntryIdx)),
    Function(FluentFunction),
}

pub trait GetEntry {
    /// Looks up a message by its string ID, and returns it if it exists.
    fn get_entry_message(&self, id: &str) -> Option<&ast::Message<&str>>;

    /// Looks up a term by its string ID, and returns it if it exists.
    fn get_entry_term(&self, id: &str) -> Option<&ast::Term<&str>>;

    /// Looks up a function by its string ID, and returns it if it exists.
    fn get_entry_function(&self, id: &str) -> Option<&FluentFunction>;
}

impl<R: Borrow<FluentResource>, M> GetEntry for FluentBundle<R, M> {
    fn get_entry_message(&self, id: &str) -> Option<&ast::Message<&str>> {
        self.entries.get(id).and_then(|ref entry| match entry {
            Entry::Message((resource_idx, entry_idx)) => {
                let res = self.resources.get(*resource_idx)?.borrow();
                if let ast::Entry::Message(ref msg) = res.get_entry(*entry_idx)? {
                    Some(msg)
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    fn get_entry_term(&self, id: &str) -> Option<&ast::Term<&str>> {
        self.entries.get(id).and_then(|ref entry| match entry {
            Entry::Term((resource_idx, entry_idx)) => {
                let res = self.resources.get(*resource_idx)?.borrow();
                if let ast::Entry::Term(ref msg) = res.get_entry(*entry_idx)? {
                    Some(msg)
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    fn get_entry_function(&self, id: &str) -> Option<&FluentFunction> {
        self.entries.get(id).and_then(|ref entry| match entry {
            Entry::Function(function) => Some(function),
            _ => None,
        })
    }
}

pub type FluentFunction = Box<dyn FluentFunctionObject + Send + Sync>;

pub trait FluentFunctionScope<'bundle> {
    fn get_message(&self, id: &str) -> Option<FluentMessage<'bundle>>;

    fn format_message(
        &mut self,
        pattern: &'bundle ast::Pattern<&'bundle str>,
        args: Option<FluentArgs<'bundle>>,
    ) -> FluentValue<'bundle>;
}

/// Implement custom function that retrieves execution scope information
pub trait FluentFunctionObject {
    fn call<'bundle>(
        &self,
        scope: &mut dyn FluentFunctionScope<'bundle>,
        positional: &[FluentValue<'bundle>],
        named: &FluentArgs<'bundle>,
    ) -> FluentValue<'bundle>;
}

impl<F> FluentFunctionObject for F
where
    F: for<'a> Fn(&[FluentValue<'a>], &FluentArgs) -> FluentValue<'a> + Sync + Send + 'static,
{
    fn call<'bundle>(
        &self,
        scope: &mut dyn FluentFunctionScope,
        positional: &[FluentValue<'bundle>],
        named: &FluentArgs,
    ) -> FluentValue<'bundle> {
        let _ = scope;
        self(positional, named)
    }
}
