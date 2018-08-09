//! `MessageContext` is a collection of localization messages in Fluent.
//!
//! It stores a list of messages in a single locale which can reference one another, use the same
//! internationalization formatters, functions, environmental variables and are expected to be used
//! together.

use std::cell::RefCell;
use std::collections::hash_map::{Entry as HashEntry, HashMap};

use super::errors::FluentError;
use super::resolve::{Env, ResolveValue};
use super::types::FluentValue;
use fluent_locale::{negotiate_languages, NegotiationStrategy};
use fluent_syntax::ast;
use fluent_syntax::parser::parse;
use intl_pluralrules::{IntlPluralRules, PluralRuleType};

pub enum Entry<'ctx> {
    Message(ast::Message),
    Term(ast::Term),
    Function(
        Box<
            'ctx + Fn(&[Option<FluentValue>], &HashMap<String, FluentValue>) -> Option<FluentValue>,
        >,
    ),
}

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
    pub locales: Vec<String>,
    pub entries: HashMap<String, Entry<'ctx>>,
    pub plural_rules: IntlPluralRules,
}

impl<'ctx> MessageContext<'ctx> {
    pub fn new<S: ToString>(locales: &[S]) -> MessageContext {
        let locales = locales
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let pr_locale = negotiate_languages(
            &locales,
            IntlPluralRules::get_locales(PluralRuleType::CARDINAL),
            Some("en"),
            &NegotiationStrategy::Lookup,
        )[0]
            .to_owned();

        let pr = IntlPluralRules::create(&pr_locale, PluralRuleType::CARDINAL).unwrap();
        MessageContext {
            locales,
            entries: HashMap::new(),
            plural_rules: pr,
        }
    }

    pub fn has_message(&self, id: &str) -> bool {
        self.entries.get(id).map_or(false, |id| {
            if let Entry::Message(_) = id {
                true
            } else {
                false
            }
        })
    }

    pub fn get_message(&self, id: &str) -> Option<&ast::Message> {
        self.entries.get(id).and_then(|id| {
            if let Entry::Message(ref msg) = id {
                Some(msg)
            } else {
                None
            }
        })
    }

    pub fn get_term(&self, id: &str) -> Option<&ast::Term> {
        self.entries.get(id).and_then(|id| {
            if let Entry::Term(ref term) = id {
                Some(term)
            } else {
                None
            }
        })
    }

    pub fn add_function<F>(&mut self, id: &str, func: F) -> Result<(), FluentError>
    where
        F: 'ctx + Fn(&[Option<FluentValue>], &HashMap<String, FluentValue>) -> Option<FluentValue>,
    {
        match self.entries.entry(id.to_owned()) {
            HashEntry::Vacant(entry) => {
                entry.insert(Entry::Function(Box::new(func)));
                Ok(())
            }
            HashEntry::Occupied(_) => Err(FluentError::Overriding {
                kind: "function",
                id: id.to_owned(),
            }),
        }
    }

    pub fn get_function(
        &self,
        id: &str,
    ) -> Option<
        &Box<
            'ctx + Fn(&[Option<FluentValue>], &HashMap<String, FluentValue>) -> Option<FluentValue>,
        >,
    > {
        self.entries.get(id).and_then(|id| {
            if let Entry::Function(ref func) = id {
                Some(func)
            } else {
                None
            }
        })
    }

    pub fn add_messages(&mut self, source: &str) -> Result<(), FluentError> {
        let res = parse(source).unwrap_or_else(|x| x.0);

        for entry in res.body {
            let id = match entry {
                ast::Entry::Message(ast::Message { ref id, .. }) => id.name.clone(),
                ast::Entry::Term(ast::Term { ref id, .. }) => id.name.clone(),
                _ => continue,
            };

            let (entry, kind) = match entry {
                ast::Entry::Message(message) => (Entry::Message(message), "message"),
                ast::Entry::Term(term) => (Entry::Term(term), "term"),
                _ => continue,
            };

            match self.entries.entry(id.clone()) {
                HashEntry::Vacant(empty) => {
                    empty.insert(entry);
                }
                HashEntry::Occupied(_) => {
                    return Err(FluentError::Overriding { kind, id });
                }
            }
        }

        Ok(())
    }

    pub fn format(&self, path: &str, args: Option<&HashMap<&str, FluentValue>>) -> Option<String> {
        let env = Env {
            ctx: self,
            args,
            travelled: RefCell::new(Vec::new()),
        };

        // `path` may be a simple message identifier (`identifier`) or a path to
        // an attribute of a message (`identifier.attrname`).
        let mut parts = path.split('.');

        // Retrieve the message by id from the context.
        let message_id = parts.next()?;
        let message = self.get_message(message_id)?;

        // Check the second and the third part of the path. The second part may
        // be an attribute name. If the third part is present, the path is
        // invalid and contains more than one period (e.g. `foo.bar.baz`).
        match (parts.next(), parts.next()) {
            (None, None) => message.to_value(&env).map(|value| value.format(self)).ok(),
            (Some(attr_name), None) => message.attributes.as_ref().and_then(|attributes| {
                for attribute in attributes {
                    if attribute.id.name == attr_name {
                        return attribute
                            .to_value(&env)
                            .map(|value| value.format(self))
                            .ok();
                    }
                }
                None
            }),
            _ => None,
        }
    }
}
