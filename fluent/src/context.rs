//! `FluentBundle` is a collection of localization messages in Fluent.
//!
//! It stores a list of messages in a single locale which can reference one another, use the same
//! internationalization formatters, functions, environmental variables and are expected to be used
//! together.

use std::cell::RefCell;
use std::collections::hash_map::{Entry as HashEntry, HashMap};

use super::entry::{Entry, GetEntry};
use super::errors::FluentError;
use super::resolve::{Env, ResolveValue};
use super::resource::FluentResource;
use super::types::FluentValue;
use fluent_locale::{negotiate_languages, NegotiationStrategy};
use fluent_syntax::ast;
use intl_pluralrules::{IntlPluralRules, PluralRuleType};

#[derive(Debug)]
pub struct Message {
    pub value: Option<String>,
    pub attributes: HashMap<String, String>,
}

/// `FluentBundle` is a collection of localization messages which are meant to be used together
/// in a single view, widget or any other UI abstraction.
///
/// # `FluentBundle` Life-cycle
///
/// To create a context, call `FluentBundle::new` with a locale list that represents the best
/// possible fallback chain for a given locale.  The simplest case is a one-locale list.
///
/// Next, call `add_messages` one or more times, supplying translations in the FTL syntax. The
/// `FluentBundle` instance is now ready to be used for localization.
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
/// `FluentBundle` stores messages in a single locale, but keeps a locale fallback chain for the
/// purpose of language negotiation with i18n formatters. For instance, if date and time formatting
/// are not available in the first locale, `FluentBundle` will use its `locales` fallback chain
/// to negotiate a sensible fallback for date and time formatting.
#[allow(dead_code)]
pub struct FluentBundle<'bundle> {
    pub locales: Vec<String>,
    pub entries: HashMap<String, Entry<'bundle>>,
    pub plural_rules: IntlPluralRules,
}

impl<'bundle> FluentBundle<'bundle> {
    pub fn new<S: ToString>(locales: &[S]) -> FluentBundle {
        let locales = locales
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let pr_locale = negotiate_languages(
            &locales,
            IntlPluralRules::get_locales(PluralRuleType::CARDINAL),
            Some("en"),
            &NegotiationStrategy::Lookup,
        )[0].to_owned();

        let pr = IntlPluralRules::create(&pr_locale, PluralRuleType::CARDINAL).unwrap();
        FluentBundle {
            locales,
            entries: HashMap::new(),
            plural_rules: pr,
        }
    }

    pub fn has_message(&self, id: &str) -> bool {
        self.entries.get_message(id).is_some()
    }

    pub fn add_function<F>(&mut self, id: &str, func: F) -> Result<(), FluentError>
    where
        F: 'bundle
            + Fn(&[Option<FluentValue>], &HashMap<String, FluentValue>) -> Option<FluentValue>,
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

    pub fn add_messages(&mut self, source: &str) -> Result<(), FluentError> {
        let res = FluentResource::from_string(source);
        self.add_resource(res)
    }

    pub fn add_resource(&mut self, res: FluentResource) -> Result<(), FluentError> {
        for entry in res.ast.body {
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
            bundle: self,
            args,
            travelled: RefCell::new(Vec::new()),
        };

        // `path` may be a simple message identifier (`identifier`) or a path to
        // an attribute of a message (`identifier.attrname`).
        let mut parts = path.split('.');

        // Retrieve the message by id from the context.
        let message_id = parts.next()?;
        let message = self.entries.get_message(message_id)?;

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

    pub fn format_message(
        &self,
        message_id: &str,
        args: Option<&HashMap<&str, FluentValue>>,
    ) -> Option<Message> {
        let env = Env {
            bundle: self,
            args,
            travelled: RefCell::new(Vec::new()),
        };
        let message = self.entries.get_message(message_id)?;

        // XXX: We should report errors in formatting
        let value = message.to_value(&env).map(|value| value.format(self)).ok();

        let mut attributes = HashMap::new();

        if let Some(ref attrs) = message.attributes {
            for attr in attrs {
                // XXX: We should report errors in formatting
                attr.to_value(&env)
                    .map(|value| value.format(self))
                    .map(|val| {
                        attributes.insert(attr.id.name.to_owned(), val);
                    });
            }
        }

        Some(Message { value, attributes })
    }
}
