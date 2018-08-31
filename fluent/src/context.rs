//! `FluentBundle` is a collection of localization messages in Fluent.
//!
//! It stores a list of messages in a single locale which can reference one another, use the same
//! internationalization formatters, functions, environmental variables and are expected to be used
//! together.

use std::cell::RefCell;
use std::collections::hash_map::{Entry as HashEntry, HashMap};

use super::entry::{Entry, GetEntry};
pub use super::errors::FluentError;
use super::resolve::{Env, ResolveValue};
use super::resource::FluentResource;
use super::types::FluentValue;
use fluent_locale::{negotiate_languages, NegotiationStrategy};
use fluent_syntax::ast;
use intl_pluralrules::{IntlPluralRules, PluralRuleType};

#[derive(Debug, PartialEq)]
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

    pub fn add_messages(&mut self, source: &str) -> Result<(), Vec<FluentError>> {
        match FluentResource::from_string(source) {
            Ok(res) => self.add_resource(res),
            Err((res, err)) => {
                let mut errors: Vec<FluentError> =
                    err.into_iter().map(FluentError::ParserError).collect();

                self.add_resource(res).map_err(|err| {
                    for e in err {
                        errors.push(e);
                    }
                    errors
                })
            }
        }
    }

    pub fn add_resource(&mut self, res: FluentResource) -> Result<(), Vec<FluentError>> {
        let mut errors = vec![];

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
                    errors.push(FluentError::Overriding { kind, id });
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn format(
        &self,
        path: &str,
        args: Option<&HashMap<&str, FluentValue>>,
    ) -> Option<(String, Vec<FluentError>)> {
        let env = Env {
            bundle: self,
            args,
            travelled: RefCell::new(Vec::new()),
        };

        let mut errors = vec![];

        if let Some(ptr_pos) = path.find('.') {
            let message_id = &path[..ptr_pos];
            let message = self.entries.get_message(message_id)?;
            let attr_name = &path[(ptr_pos + 1)..];
            if let Some(ref attributes) = message.attributes {
                for attribute in attributes {
                    if attribute.id.name == attr_name {
                        match attribute.to_value(&env) {
                            Ok(val) => {
                                return Some((val.format(self), errors));
                            }
                            Err(err) => {
                                errors.push(FluentError::ResolverError(err));
                                // XXX: In the future we'll want to get the partial
                                // value out of resolver and return it here.
                                // We also expect to get a Vec or errors out of resolver.
                                return Some((path.to_string(), errors));
                            }
                        }
                    }
                }
            }
        } else {
            let message_id = path;
            let message = self.entries.get_message(message_id)?;
            match message.to_value(&env) {
                Ok(val) => {
                    let s = val.format(self);
                    return Some((s, errors));
                }
                Err(err) => {
                    errors.push(FluentError::ResolverError(err));
                    return Some((message_id.to_string(), errors));
                }
            }
        }

        None
    }

    pub fn format_message(
        &self,
        message_id: &str,
        args: Option<&HashMap<&str, FluentValue>>,
    ) -> Option<(Message, Vec<FluentError>)> {
        let mut errors = vec![];

        let env = Env {
            bundle: self,
            args,
            travelled: RefCell::new(Vec::new()),
        };
        let message = self.entries.get_message(message_id)?;

        let value = match message.to_value(&env) {
            Ok(value) => Some(value.format(self)),
            Err(err) => {
                errors.push(FluentError::ResolverError(err));
                None
            }
        };

        let mut attributes = HashMap::new();

        if let Some(ref attrs) = message.attributes {
            for attr in attrs {
                match attr.to_value(&env) {
                    Ok(value) => {
                        let val = value.format(self);
                        attributes.insert(attr.id.name.to_owned(), val);
                    }
                    Err(err) => {
                        errors.push(FluentError::ResolverError(err));
                    }
                }
            }
        }

        Some((Message { value, attributes }, errors))
    }
}
