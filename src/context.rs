//! Context is a collection of localization messages in Fluent.
//!
//! It stores a list of messages in a single locale which can reference one another,
//! use same internationalization formatters, functions, environmental variables
//! and are expected to be used together.
//!
//! The main structure is called `MessageContext`.

use std::collections::HashMap;

use super::syntax::ast;
use super::syntax::parse;
use super::types::FluentValue;
use super::resolve::{Env, ResolveValue};

/// MessageContext is a collection of localization messages that are meant to
/// be used together in a single localization context.
///
/// An example of such structure is a single UI widget, panel or view.
///
/// # MessageContext life-cycle
///
/// To create a context, call `MessageContext::new` with a locale list that
/// represents the best possible fallback chain for a given locale.
/// The simplest case is a one-locale list.
///
/// Next, call `add_messages` one or more times, providing it with a list
/// of localization messages formatted using FTL syntax.
///
/// From that moment, the `MessageContext` object is ready to be used for localization.
///
/// At any point when a translation is neeeded, call `get_message` to retrieve a
/// `fluent::context::Message` structure and then `format` within the context.
///
/// The result is always a single string that should be displayed in the UI.
/// Like all internationalization operations, the resulting string is not to
/// be examined for testing purposes or altered before displaying in anyway.
///
/// It is recommended to treat such string as opaque from the perspective of the
/// program and use it only to display localized messages.
///
///
/// # Locale Fallback Chain
///
/// MessageContext stores messages in a single locale, but keeps a locale
/// fallback chain for language negotaition with internationalization formatters
/// purposes. For example if date and time formatting is impossible in the main
/// language, MessageContext can negotiate with date and time internationalization
/// formatter to use a sensible fallback locale based on the fallback chain.
#[allow(dead_code)]
pub struct MessageContext<'ctx> {
    pub locales: &'ctx [&'ctx str],
    messages: HashMap<String, ast::Message>,
}

impl<'ctx> MessageContext<'ctx> {
    pub fn new(locales: &'ctx [&'ctx str]) -> MessageContext {
        MessageContext {
            locales,
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
        resolvable.to_value(&env).map(|value| value.format(self))
    }
}
