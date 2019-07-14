//! `FluentBundle` is a collection of localization messages in Fluent.
//!
//! It stores a list of messages in a single locale which can reference one another, use the same
//! internationalization formatters, functions, scopeironmental variables and are expected to be used
//! together.

use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::hash_map::{Entry as HashEntry, HashMap};

use fluent_locale::{negotiate_languages, NegotiationStrategy};
use fluent_syntax::ast;
use intl_pluralrules::{IntlPluralRules, PluralRuleType};

use crate::entry::Entry;
use crate::entry::GetEntry;
use crate::errors::FluentError;
use crate::resolve::{resolve_value_for_entry, Scope};
use crate::resource::FluentResource;
use crate::types::FluentValue;

#[derive(Debug, PartialEq)]
pub struct Message<'m> {
    pub value: Option<Cow<'m, str>>,
    pub attributes: HashMap<&'m str, Cow<'m, str>>,
}

/// A collection of localization messages for a single locale, which are meant
/// to be used together in a single view, widget or any other UI abstraction.
///
/// # Examples
///
/// ```
/// use fluent_bundle::{FluentBundle, FluentResource, FluentValue};
/// use std::collections::HashMap;
///
/// let ftl_string = String::from("intro = Welcome, { $name }.");
/// let resource = FluentResource::try_new(ftl_string)
///     .expect("Could not parse an FTL string.");
///
/// let mut bundle = FluentBundle::new(&["en-US"]);
/// bundle.add_resource(&resource)
///     .expect("Failed to add FTL resources to the bundle.");
///
/// let mut args = HashMap::new();
/// args.insert("name", FluentValue::from("Rustacean"));
///
/// let (value, _) = bundle.format("intro", Some(&args))
///     .expect("Failed to format a message.");
/// assert_eq!(&value, "Welcome, \u{2068}Rustacean\u{2069}.");
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
/// retrieve the translated string. Alternately, [`compound`] provides a convenient way of
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
/// [`compound`]: ./struct.FluentBundle.html#method.compound
/// [`add_resource`]: ./struct.FluentBundle.html#method.add_resource
/// [`Option<T>`]: http://doc.rust-lang.org/std/option/enum.Option.html
pub struct FluentBundle<R> {
    pub locales: Vec<String>,
    pub(crate) resources: Vec<R>,
    pub(crate) entries: HashMap<String, Entry>,
    pub(crate) plural_rules: IntlPluralRules,
    pub(crate) use_isolating: bool,
}

impl<R> FluentBundle<R> {
    /// Constructs a FluentBundle. `locales` is the fallback chain of locales
    /// to use for formatters like date and time. `locales` does not influence
    /// message selection.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_bundle::FluentBundle;
    /// use fluent_bundle::FluentResource;
    ///
    /// let mut bundle: FluentBundle<FluentResource> = FluentBundle::new(&["en-US"]);
    /// ```
    ///
    /// # Errors
    ///
    /// This will panic if no formatters can be found for the locales.
    pub fn new<S: ToString>(locales: &[S]) -> Self {
        let locales = locales
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>();
        let pr_locale = negotiate_languages(
            &locales,
            IntlPluralRules::get_locales(PluralRuleType::CARDINAL),
            Some("en"),
            &NegotiationStrategy::Lookup,
        )[0]
        .to_owned();

        let pr = IntlPluralRules::create(&pr_locale, PluralRuleType::CARDINAL)
            .expect("Failed to initialize PluralRules.");
        FluentBundle {
            locales,
            resources: vec![],
            entries: HashMap::new(),
            plural_rules: pr,
            use_isolating: true,
        }
    }

    /// Adds a resource to the bundle, returning an empty [`Result<T>`] on success.
    ///
    /// The method can take any type that can be borrowed to FluentResource:
    ///   - FluentResource
    ///   - &FluentResource
    ///   - Rc<FluentResource>
    ///   - Arc<FluentResurce>
    ///
    /// This allows the user to introduce custom resource management and share
    /// resources between instances of `FluentBundle`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_bundle::{FluentBundle, FluentResource};
    ///
    /// let ftl_string = String::from("
    /// hello = Hi!
    /// goodbye = Bye!
    /// ");
    /// let resource = FluentResource::try_new(ftl_string)
    ///     .expect("Could not parse an FTL string.");
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
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
    pub fn add_resource(&mut self, r: R) -> Result<(), Vec<FluentError>>
    where
        R: Borrow<FluentResource>,
    {
        let mut errors = vec![];

        let res = r.borrow();
        let res_pos = self.resources.len();

        for (entry_pos, entry) in res.ast().body.iter().enumerate() {
            let id = match entry {
                ast::ResourceEntry::Entry(ast::Entry::Message(ast::Message { ref id, .. }))
                | ast::ResourceEntry::Entry(ast::Entry::Term(ast::Term { ref id, .. })) => id.name,
                _ => continue,
            };

            let (entry, kind) = match entry {
                ast::ResourceEntry::Entry(ast::Entry::Message(..)) => {
                    (Entry::Message([res_pos, entry_pos]), "message")
                }
                ast::ResourceEntry::Entry(ast::Entry::Term(..)) => {
                    (Entry::Term([res_pos, entry_pos]), "term")
                }
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
        self.resources.push(r);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn set_use_isolating(&mut self, value: bool) {
        self.use_isolating = value;
    }

    /// Returns true if this bundle contains a message with the given id.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_bundle::{FluentBundle, FluentResource};
    ///
    /// let ftl_string = String::from("hello = Hi!");
    /// let resource = FluentResource::try_new(ftl_string)
    ///     .expect("Failed to parse an FTL string.");
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(&resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
    /// assert_eq!(true, bundle.has_message("hello"));
    ///
    /// ```
    pub fn has_message(&self, id: &str) -> bool
    where
        R: Borrow<FluentResource>,
    {
        self.get_message(id).is_some()
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
    /// use fluent_bundle::{FluentBundle, FluentResource, FluentValue};
    ///
    /// let ftl_string = String::from("length = { STRLEN(\"12345\") }");
    /// let resource = FluentResource::try_new(ftl_string)
    ///     .expect("Could not parse an FTL string.");
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(&resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
    ///
    /// // Register a fn that maps from string to string length
    /// bundle.add_function("STRLEN", |positional, _named| match positional {
    ///     [FluentValue::String(str)] => FluentValue::Number(str.len().to_string().into()),
    ///     _ => FluentValue::None(),
    /// }).expect("Failed to add a function to the bundle.");
    ///
    /// let (value, _) = bundle.format("length", None)
    ///     .expect("Failed to format a message.");
    /// assert_eq!(&value, "5");
    /// ```
    ///
    /// [FTL syntax guide]: https://projectfluent.org/fluent/guide/functions.html
    pub fn add_function<F: 'static>(&mut self, id: &str, func: F) -> Result<(), FluentError>
    where
        F: for<'a> Fn(&[FluentValue<'a>], &HashMap<&str, FluentValue<'a>>) -> FluentValue<'a>
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

    /// Formats the message value identified by `path` using `args` to
    /// provide variables. `path` is either a message id ("hello"), or
    /// message id plus attribute ("hello.tooltip").
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_bundle::{FluentBundle, FluentResource, FluentValue};
    /// use std::collections::HashMap;
    ///
    /// let ftl_string = String::from("intro = Welcome, { $name }.");
    /// let resource = FluentResource::try_new(ftl_string)
    ///     .expect("Could not parse an FTL string.");
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
    ///
    /// let mut args = HashMap::new();
    /// args.insert("name", FluentValue::from("Rustacean"));
    ///
    /// let (value, _) = bundle.format("intro", Some(&args))
    ///     .expect("Failed to format a message.");
    /// assert_eq!(&value, "Welcome, \u{2068}Rustacean\u{2069}.");
    ///
    /// ```
    ///
    /// An example with attributes and no args:
    ///
    /// ```
    /// use fluent_bundle::{FluentBundle, FluentResource};
    ///
    /// let ftl_string = String::from("
    /// hello =
    ///     .title = Hi!
    ///     .tooltip = This says 'Hi!'
    /// ");
    /// let resource = FluentResource::try_new(ftl_string)
    ///     .expect("Could not parse an FTL string.");
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
    ///
    /// let (value, _) = bundle.format("hello.title", None)
    ///     .expect("Failed to format a message.");
    /// assert_eq!(&value, "Hi!");
    /// ```
    ///
    /// # Errors
    ///
    /// For some cases where `format` can't find a message it will return `None`.
    ///
    /// In all other cases `format` returns a string even if it
    /// encountered errors. Generally, during partial errors `format` will
    /// use ids to replace parts of the formatted message that it could
    /// not successfuly build. For more fundamental errors `format` will return
    /// the path itself as the translation.
    ///
    /// The second term of the tuple will contain any extra error information
    /// gathered during formatting. A caller may safely ignore the extra errors
    /// if the fallback formatting policies are acceptable.
    ///
    /// ```
    /// use fluent_bundle::{FluentBundle, FluentResource};
    ///
    /// // Create a message with bad cyclic reference
    /// let ftl_string = String::from("foo = a { foo } b");
    /// let resource = FluentResource::try_new(ftl_string)
    ///     .expect("Could not parse an FTL string.");
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
    ///
    /// // The result falls back to "a foo b"
    /// let (value, _) = bundle.format("foo", None)
    ///     .expect("Failed to format a message.");
    /// assert_eq!(&value, "a foo b");
    /// ```
    pub fn format<'bundle>(
        &'bundle self,
        path: &str,
        args: Option<&'bundle HashMap<&str, FluentValue>>,
    ) -> Option<(Cow<'bundle, str>, Vec<FluentError>)>
    where
        R: Borrow<FluentResource>,
    {
        let mut scope = Scope::new(self, args);

        let mut errors = vec![];

        let string = if let Some(ptr_pos) = path.find('.') {
            let message_id = &path[..ptr_pos];
            let message = self.get_message(message_id)?;
            let attr_name = &path[(ptr_pos + 1)..];
            let attr = message
                .attributes
                .iter()
                .find(|attr| attr.id.name == attr_name)?;
            resolve_value_for_entry(&attr.value, (message, attr).into(), &mut scope).to_string()
        } else {
            let message_id = path;
            let message = self.get_message(message_id)?;
            message
                .value
                .as_ref()
                .map(|value| resolve_value_for_entry(value, message.into(), &mut scope))?
                .to_string()
        };

        for err in scope.errors {
            errors.push(err.into());
        }

        Some((string, errors))
    }

    /// Formats both the message value and attributes identified by `message_id`
    /// using `args` to provide variables. This is useful for cases where a UI
    /// element requires multiple related text fields, such as a button that has
    /// both display text and assistive text.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_bundle::{FluentBundle, FluentResource, FluentValue};
    /// use std::collections::HashMap;
    ///
    /// let ftl_string = String::from("
    /// login-input = Predefined value
    ///     .placeholder = example@email.com
    ///     .aria-label = Login input value
    ///     .title = Type your login email
    /// ");
    /// let resource = FluentResource::try_new(ftl_string)
    ///     .expect("Could not parse an FTL string.");
    /// let mut bundle = FluentBundle::new(&["en-US"]);
    /// bundle.add_resource(resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
    ///
    /// let (message, _) = bundle.compound("login-input", None)
    ///     .expect("Failed to format a message.");
    /// assert_eq!(message.value, Some("Predefined value".into()));
    /// assert_eq!(message.attributes.get("title"), Some(&"Type your login email".into()));
    /// ```
    ///
    /// # Errors
    ///
    /// For some cases where `compound` can't find a message it will return `None`.
    ///
    /// In all other cases `compound` returns a message even if it
    /// encountered errors. Generally, during partial errors `compound` will
    /// use ids to replace parts of the formatted message that it could
    /// not successfuly build. For more fundamental errors `compound` will return
    /// the path itself as the translation.
    ///
    /// The second term of the tuple will contain any extra error information
    /// gathered during formatting. A caller may safely ignore the extra errors
    /// if the fallback formatting policies are acceptable.
    pub fn compound<'bundle>(
        &'bundle self,
        message_id: &str,
        args: Option<&'bundle HashMap<&str, FluentValue>>,
    ) -> Option<(Message<'bundle>, Vec<FluentError>)>
    where
        R: Borrow<FluentResource>,
    {
        let mut scope = Scope::new(self, args);
        let mut errors = vec![];
        let message = self.get_message(message_id)?;

        let value = message
            .value
            .as_ref()
            .map(|value| resolve_value_for_entry(value, message.into(), &mut scope).to_string());

        // Setting capacity helps performance for cases with attributes,
        // but is slower than `::new` for cases without.
        // Maybe one day this will be fixed but for now let's use the trick.
        let mut attributes = if message.attributes.is_empty() {
            HashMap::new()
        } else {
            HashMap::with_capacity(message.attributes.len())
        };

        for attr in message.attributes.iter() {
            let val = resolve_value_for_entry(&attr.value, (message, attr).into(), &mut scope);
            attributes.insert(attr.id.name, val.to_string());
        }

        for err in scope.errors {
            errors.push(err.into());
        }

        Some((Message { value, attributes }, errors))
    }
}
