//! `MessageContext` is a collection of localization messages in Fluent.
//!
//! It stores a list of messages in a single locale which can reference one another, use the same
//! internationalization formatters, functions, environmental variables and are expected to be used
//! together.

use std::collections::HashMap;

use super::syntax::ast;
use super::syntax::parse;
use super::types::FluentValue;
use super::resolve::{Env, ResolveValue};

/// `MessageContext` is a collection of localization messages which are meant to be used together
/// in a single view, widget or any other UI abstraction.
///
/// # `MessageContext` Life-cycle
///
/// To create a context, call `MessageContext::new` with a locale list that represents the best
/// possible fallback chain for a given locale.  The simplest case is a one-locale list.
///
/// Next, call `add_messages` one or more times, supplying translations in the FTL syntax. The
/// `MessageContext` instance is now ready to be used for localization.
///
/// To format a translation, call `get_message` to retrieve a `fluent::context::Message` structure
/// and then `format` it within the context.
///
/// The result is an Option wrapping a single string that should be displayed in the UI.  It is
/// recommended to treat the result as opaque from the perspective of the program and use it only
/// to display localized messages. Do not examine it or alter in any way before displaying.  This
/// is a general good practice as far as all internationalization operations are concerned.
///
///
/// # Locale Fallback Chain
///
/// `MessageContext` stores messages in a single locale, but keeps a locale fallback chain for the
/// purpose of language negotiation with i18n formatters. For instance, if date and time formatting
/// are not available in the first locale, `MessageContext` will use its `locales` fallback chain
/// to negotiate a sensible fallback for date and time formatting.
#[allow(dead_code)]
pub struct MessageContext<'ctx> {
    pub locales: &'ctx [&'ctx str],
    messages: HashMap<String, ast::Message>,
    terms: HashMap<String, ast::Term>,
    functions: HashMap<
        String,
        Box<Fn(&[Option<FluentValue>], &HashMap<String, FluentValue>) -> Option<FluentValue>>,
    >,
}

impl<'ctx> MessageContext<'ctx> {
    pub fn new(locales: &'ctx [&'ctx str]) -> MessageContext {
        MessageContext {
            locales,
            messages: HashMap::new(),
            terms: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn has_message(&self, id: &str) -> bool {
        self.messages.contains_key(id)
    }

    pub fn get_message(&self, id: &str) -> Option<&ast::Message> {
        self.messages.get(id)
    }

    pub fn get_term(&self, id: &str) -> Option<&ast::Term> {
        self.terms.get(id)
    }

    pub fn add_function<F>(&mut self, id: &str, func: F)
    where
        F: 'static
            + Fn(&[Option<FluentValue>], &HashMap<String, FluentValue>) -> Option<FluentValue>,
    {
        self.functions.insert(id.to_string(), Box::new(func));
    }

    pub fn get_function(
        &self,
        id: &str,
    ) -> Option<
        &Box<Fn(&[Option<FluentValue>], &HashMap<String, FluentValue>) -> Option<FluentValue>>,
    > {
        self.functions.get(id)
    }

    pub fn add_messages(&mut self, source: &str) {
        let res = parse(source).unwrap_or_else(|x| x.0);

        for entry in res.body {
            let id = match entry {
                ast::Entry::Message(ast::Message { ref id, .. }) => id.name.clone(),
                ast::Entry::Term(ast::Term { ref id, .. }) => id.name.clone(),
                _ => continue,
            };

            match entry {
                ast::Entry::Message(message) => {
                    self.messages.insert(id, message);
                }
                ast::Entry::Term(term) => {
                    self.terms.insert(id, term);
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
        resolvable.to_value(&env).map(|value| value.format(self))
    }
}
