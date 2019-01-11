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

/// A collection of localization messages for a single locale, which are meant
/// to be used together in a single view, widget or any other UI abstraction.
///
/// # Examples
///
/// ```
/// use fluent_bundle::bundle::FluentBundle;
/// use fluent_bundle::resource::FluentResource;
/// use fluent_bundle::types::FluentValue;
/// use std::collections::HashMap;
///
/// let resource = FluentResource::try_new("intro = Welcome, { $name }.".to_string()).unwrap();
/// let mut bundle = FluentBundle::new(&["en-US"]);
/// bundle.add_resource(&resource);
///
/// let mut args = HashMap::new();
/// args.insert("name", FluentValue::from("Rustacean"));
///
/// let value = bundle.format("intro", Some(&args));
/// assert_eq!(value, Some(("Welcome, Rustacean.".to_string(), vec![])));
///
/// ```
///
/// # `FluentBundle` Life Cycle
///
/// To create a bundle, call [`FluentBundle::new`] with a locale list that represents the best
/// possible fallback chain for a given locale. The simplest case is a one-locale list.
///
/// Next, call [`add_resource`] one or more times, supplying translations in the FTL syntax. The
/// `FluentBundle` instance is now ready to be used for localization.
///
/// To format a translation, call [`format`] with the path of a message or attribute in order to
/// retrieve the translated string. Alternately, [`format_message`] provides a convenient way of
/// formatting all attributes of a message at once.
///
/// The result of `format` is an [`Option<T>`] wrapping a `(String, Vec<FluentError>)`. On success,
/// the string is a formatted value that should be displayed in the UI. It is
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
///
/// [`add_resource`]: ./struct.FluentBundle.html#method.add_resource
/// [`FluentBundle::new`]: ./struct.FluentBundle.html#method.new
/// [`fluent::bundle::Message`]: ./struct.FluentBundle.html#method.new
/// [`format`]: ./struct.FluentBundle.html#method.format
/// [`format_message`]: ./struct.FluentBundle.html#method.format_message
/// [`add_resource`]: ./struct.FluentBundle.html#method.add_resource
/// [`Option<T>`]: http://doc.rust-lang.org/std/option/enum.Option.html
#[allow(dead_code)]
pub struct FluentBundle<'bundle> {
    pub locales: Vec<String>,
    pub entries: HashMap<String, Entry<'bundle>>,
    pub plural_rules: IntlPluralRules,
}

impl<'bundle> FluentBundle<'bundle> {
    /// Constructs a FluentBundle. `locales` is the fallback chain of locales
    /// to use for formatters like date and time. `locales` does not influence
    /// message selection.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_bundle::bundle::FluentBundle;
    ///
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// ```
    ///
    /// # Errors
    ///
    /// This will panic if no formatters can be found for the locales.
    pub fn new<'a, S: ToString>(locales: &'a [S]) -> FluentBundle<'bundle> {
        let locales = locales.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let pr_locale = negotiate_languages(
            &locales,
            IntlPluralRules::get_locales(PluralRuleType::CARDINAL),
            Some("en"),
            &NegotiationStrategy::Lookup,
        )[0]
        .to_owned();

        let pr = IntlPluralRules::create(&pr_locale, PluralRuleType::CARDINAL).unwrap();
        FluentBundle {
            locales,
            entries: HashMap::new(),
            plural_rules: pr,
        }
    }

    /// Returns true if this bundle contains a message with the given id.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_bundle::bundle::FluentBundle;
    /// use fluent_bundle::resource::FluentResource;
    ///
    /// let resource = FluentResource::try_new("hello = Hi!".to_string()).unwrap();
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(&resource);
    /// assert_eq!(true, bundle.has_message("hello"));
    /// 
    /// ```
    pub fn has_message(&self, id: &str) -> bool {
        self.entries.get_message(id).is_some()
    }

    /// Makes the provided rust function available to messages with the name `id`. See
    /// the [FTL syntax guide] to learn how these are used in messages.
    ///
    /// FTL functions accept both positional and named args. The rust function you
    /// provide therefore has two parameters: a slice of values for the positional
    /// args, and a HashMap of values for named args.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_bundle::bundle::FluentBundle;
    /// use fluent_bundle::resource::FluentResource;
    /// use fluent_bundle::types::FluentValue;
    ///
    /// let resource = FluentResource::try_new("length = { STRLEN(\"12345\") }".to_string()).unwrap();
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(&resource);
    ///
    /// // Register a fn that maps from string to string length
    /// bundle.add_function("STRLEN", |positional, _named| match positional {
    ///     [Some(FluentValue::String(str))] => Some(FluentValue::Number(str.len().to_string())),
    ///     _ => None,
    /// }).unwrap();
    ///
    /// let (value, _) = bundle.format("length", None).unwrap();
    /// assert_eq!(&value, "5");
    /// ```
    ///
    /// [FTL syntax guide]: https://projectfluent.org/fluent/guide/functions.html
    pub fn add_function<F>(&mut self, id: &str, func: F) -> Result<(), FluentError>
    where
        F: 'bundle
            + Fn(&[Option<FluentValue>], &HashMap<String, FluentValue>) -> Option<FluentValue>
            + Sync
            + Send,
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

    /// Adds the message or messages, in [FTL syntax], to the bundle, returning an
    /// empty [`Result<T>`] on success.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_bundle::bundle::FluentBundle;
    /// use fluent_bundle::resource::FluentResource;
    ///
    /// let resource = FluentResource::try_new("
    /// hello = Hi!
    /// goodbye = Bye!
    /// ".to_string()).unwrap();
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(&resource);
    /// assert_eq!(true, bundle.has_message("hello"));
    /// ```
    ///
    /// # Whitespace
    ///
    /// Message ids must have no leading whitespace. Message values that span
    /// multiple lines must have leading whitespace on all but the first line. These
    /// are standard FTL syntax rules that may prove a bit troublesome in source
    /// code formatting. The [`indoc!`] crate can help with stripping extra indentation
    /// if you wish to indent your entire message.
    ///
    /// [FTL syntax]: https://projectfluent.org/fluent/guide/
    /// [`indoc!`]: https://github.com/dtolnay/indoc
    /// [`Result<T>`]: https://doc.rust-lang.org/std/result/enum.Result.html
    pub fn add_resource(&mut self, res: &'bundle FluentResource) -> Result<(), Vec<FluentError>> {
        let mut errors = vec![];

        for entry in &res.ast().body {
            let id = match entry {
                ast::ResourceEntry::Entry(ast::Entry::Message(ast::Message { ref id, .. }))
                | ast::ResourceEntry::Entry(ast::Entry::Term(ast::Term { ref id, .. })) => id.name,
                _ => continue,
            };

            let (entry, kind) = match entry {
                ast::ResourceEntry::Entry(ast::Entry::Message(message)) => {
                    (Entry::Message(message), "message")
                }
                ast::ResourceEntry::Entry(ast::Entry::Term(term)) => (Entry::Term(term), "term"),
                _ => continue,
            };

            match self.entries.entry(id.to_string()) {
                HashEntry::Vacant(empty) => {
                    empty.insert(entry);
                }
                HashEntry::Occupied(_) => {
                    errors.push(FluentError::Overriding {
                        kind,
                        id: id.to_string(),
                    });
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Formats the message value identified by `path` using `args` to
    /// provide variables. `path` is either a message id ("hello"), or
    /// message id plus attribute ("hello.tooltip").
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_bundle::bundle::FluentBundle;
    /// use fluent_bundle::resource::FluentResource;
    /// use fluent_bundle::types::FluentValue;
    /// use std::collections::HashMap;
    ///
    /// let resource = FluentResource::try_new("intro = Welcome, { $name }.".to_string()).unwrap();
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(&resource);
    ///
    /// let mut args = HashMap::new();
    /// args.insert("name", FluentValue::from("Rustacean"));
    ///
    /// let value = bundle.format("intro", Some(&args));
    /// assert_eq!(value, Some(("Welcome, Rustacean.".to_string(), vec![])));
    ///
    /// ```
    ///
    /// An example with attributes and no args:
    ///
    /// ```
    /// use fluent_bundle::bundle::FluentBundle;
    /// use fluent_bundle::resource::FluentResource;
    ///
    /// let resource = FluentResource::try_new("
    /// hello =
    ///     .title = Hi!
    ///     .tooltip = This says 'Hi!'
    /// ".to_string()).unwrap();
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(&resource);
    ///
    /// let value = bundle.format("hello.title", None);
    /// assert_eq!(value, Some(("Hi!".to_string(), vec![])));
    /// ```
    ///
    /// # Errors
    ///
    /// For some cases where `format` can't find a message it will return `None`.
    /// 
    /// In all other cases `format` returns a string even if it
    /// encountered errors. Generally, during partial errors `format` will
    /// use `'___'` to replace parts of the formatted message that it could
    /// not successfuly build. For more fundamental errors `format` will return
    /// the path itself as the translation.
    /// 
    /// The second term of the tuple will contain any extra error information
    /// gathered during formatting. A caller may safely ignore the extra errors
    /// if the fallback formatting policies are acceptable.
    /// 
    /// ```
    /// use fluent_bundle::bundle::FluentBundle;
    /// use fluent_bundle::resource::FluentResource;
    /// 
    /// // Create a message with bad cyclic reference
    /// let mut res = FluentResource::try_new("foo = a { foo } b".to_string()).unwrap();
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(&res);
    /// 
    /// // The result falls back to "___"
    /// let value = bundle.format("foo", None);
    /// assert_eq!(value, Some(("___".to_string(), vec![])));
    /// ```
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
            for attribute in message.attributes.iter() {
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

    /// Formats both the message value and attributes identified by `message_id`
    /// using `args` to provide variables. This is useful for cases where a UI
    /// element requires multiple related text fields, such as a button that has
    /// both display text and assistive text.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_bundle::bundle::FluentBundle;
    /// use fluent_bundle::resource::FluentResource;
    /// use fluent_bundle::types::FluentValue;
    /// use std::collections::HashMap;
    ///
    /// let mut res = FluentResource::try_new("
    /// login-input = Predefined value
    ///     .placeholder = example@email.com
    ///     .aria-label = Login input value
    ///     .title = Type your login email".to_string()).unwrap();
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(&res);
    /// 
    /// let (message, _) = bundle.format_message("login-input", None).unwrap();
    /// assert_eq!(message.value, Some("Predefined value".to_string()));
    /// assert_eq!(message.attributes.get("title"), Some(&"Type your login email".to_string()));
    /// ```
    ///
    /// # Errors
    ///
    /// For some cases where `format_message` can't find a message it will return `None`.
    /// 
    /// In all other cases `format_message` returns a message even if it
    /// encountered errors. Generally, during partial errors `format_message` will
    /// use `'___'` to replace parts of the formatted message that it could
    /// not successfuly build. For more fundamental errors `format_message` will return
    /// the path itself as the translation.
    /// 
    /// The second term of the tuple will contain any extra error information
    /// gathered during formatting. A caller may safely ignore the extra errors
    /// if the fallback formatting policies are acceptable.
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

        for attr in message.attributes.iter() {
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

        Some((Message { value, attributes }, errors))
    }
}
