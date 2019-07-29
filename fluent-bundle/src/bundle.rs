//! `FluentBundle` is a collection of localization messages in Fluent.
//!
//! It stores a list of messages in a single locale which can reference one another, use the same
//! internationalization formatters, functions, scopeironmental variables and are expected to be used
//! together.

use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::hash_map::{Entry as HashEntry, HashMap};
use std::default::Default;

use fluent_locale::{negotiate_languages, NegotiationStrategy};
use fluent_syntax::ast;
use intl_pluralrules::{IntlPluralRules, PluralRuleType};
use unic_langid::langid;
use unic_langid::LanguageIdentifier;

use crate::entry::Entry;
use crate::entry::GetEntry;
use crate::errors::FluentError;
use crate::resolve::{ResolveValue, Scope};
use crate::resource::FluentResource;
use crate::types::FluentValue;

#[derive(Debug, PartialEq)]
pub struct FluentMessage<'m> {
    pub value: Option<&'m ast::Pattern<'m>>,
    pub attributes: HashMap<&'m str, &'m ast::Pattern<'m>>,
}

pub type FluentArgs<'args> = HashMap<&'args str, FluentValue<'args>>;

/// A collection of localization messages for a single locale, which are meant
/// to be used together in a single view, widget or any other UI abstraction.
///
/// # Examples
///
/// ```
/// use fluent_bundle::{FluentBundle, FluentResource, FluentValue};
/// use std::collections::HashMap;
/// use unic_langid::langid;
///
/// let ftl_string = String::from("intro = Welcome, { $name }.");
/// let resource = FluentResource::try_new(ftl_string)
///     .expect("Could not parse an FTL string.");
///
/// let langid_en = langid!("en-US");
/// let mut bundle = FluentBundle::new(&[langid_en]);
/// bundle.add_resource(&resource)
///     .expect("Failed to add FTL resources to the bundle.");
///
/// let mut args = HashMap::new();
/// args.insert("name", FluentValue::from("Rustacean"));
///
/// let msg = bundle.get_message("intro").expect("Message doesn't exist.");
/// let mut errors = vec![];
/// let pattern = msg.value.expect("Message has no value.");
/// let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);
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
/// To format a translation, call [`get_message`] to retrieve a [`FluentMessage`],
/// and then call [`format_pattern`] on the message value or attribute in order to
/// retrieve the translated string.
///
/// The result of `format_pattern` is an [`Cow<str>`]. It is
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
/// [`FluentMessage`]: ./struct.FluentMessage.html
/// [`get_message`]: ./struct.FluentBundle.html#method.get_message
/// [`format_pattern`]: ./struct.FluentBundle.html#method.format_pattern
/// [`add_resource`]: ./struct.FluentBundle.html#method.add_resource
/// [`Cow<str>`]: http://doc.rust-lang.org/std/borrow/enum.Cow.html
pub struct FluentBundle<R> {
    pub locales: Vec<LanguageIdentifier>,
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
    /// use unic_langid::langid;
    ///
    /// let langid_en = langid!("en-US");
    /// let mut bundle: FluentBundle<FluentResource> = FluentBundle::new(&[langid_en]);
    /// ```
    ///
    /// # Errors
    ///
    /// This will panic if no formatters can be found for the locales.
    pub fn new<'a, L: 'a + Into<LanguageIdentifier> + PartialEq + Clone>(
        locales: impl IntoIterator<Item = &'a L>,
    ) -> Self {
        let locales = locales
            .into_iter()
            .map(|s| s.clone().into())
            .collect::<Vec<_>>();
        let default_langid = langid!("en");
        let pr_locale = negotiate_languages(
            &locales,
            &IntlPluralRules::get_locales(PluralRuleType::CARDINAL),
            Some(&default_langid),
            NegotiationStrategy::Lookup,
        )[0]
        .clone();

        let pr = IntlPluralRules::create(pr_locale, PluralRuleType::CARDINAL)
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
    /// use unic_langid::langid;
    ///
    /// let ftl_string = String::from("
    /// hello = Hi!
    /// goodbye = Bye!
    /// ");
    /// let resource = FluentResource::try_new(ftl_string)
    ///     .expect("Could not parse an FTL string.");
    /// let langid_en = langid!("en-US");
    /// let mut bundle = FluentBundle::new(&[langid_en]);
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
    /// use unic_langid::langid;
    ///
    /// let ftl_string = String::from("hello = Hi!");
    /// let resource = FluentResource::try_new(ftl_string)
    ///     .expect("Failed to parse an FTL string.");
    /// let langid_en = langid!("en-US");
    /// let mut bundle = FluentBundle::new(&[langid_en]);
    /// bundle.add_resource(&resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
    /// assert_eq!(true, bundle.has_message("hello"));
    ///
    /// ```
    pub fn has_message(&self, id: &str) -> bool
    where
        R: Borrow<FluentResource>,
    {
        self.get_entry_message(id).is_some()
    }

    pub fn get_message(&self, id: &str) -> Option<FluentMessage>
    where
        R: Borrow<FluentResource>,
    {
        let message = self.get_entry_message(id)?;
        let value = message.value.as_ref();
        let mut attributes = if message.attributes.is_empty() {
            HashMap::new()
        } else {
            HashMap::with_capacity(message.attributes.len())
        };

        for attr in message.attributes.iter() {
            attributes.insert(attr.id.name, &attr.value);
        }
        Some(FluentMessage { value, attributes })
    }

    pub fn format_pattern<'bundle>(
        &'bundle self,
        pattern: &'bundle ast::Pattern,
        args: Option<&'bundle FluentArgs>,
        errors: &mut Vec<FluentError>,
    ) -> Cow<'bundle, str>
    where
        R: Borrow<FluentResource>,
    {
        let mut scope = Scope::new(self, args);
        let result = pattern.resolve(&mut scope).to_string();

        for err in scope.errors {
            errors.push(err.into());
        }

        result
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
    /// use unic_langid::langid;
    ///
    /// let ftl_string = String::from("length = { STRLEN(\"12345\") }");
    /// let resource = FluentResource::try_new(ftl_string)
    ///     .expect("Could not parse an FTL string.");
    /// let langid_en = langid!("en-US");
    /// let mut bundle = FluentBundle::new(&[langid_en]);
    /// bundle.add_resource(&resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
    ///
    /// // Register a fn that maps from string to string length
    /// bundle.add_function("STRLEN", |positional, _named| match positional {
    ///     [FluentValue::String(str)] => FluentValue::Number(str.len().to_string().into()),
    ///     _ => FluentValue::None,
    /// }).expect("Failed to add a function to the bundle.");
    ///
    /// let msg = bundle.get_message("length").expect("Message doesn't exist.");
    /// let mut errors = vec![];
    /// let pattern = msg.value.expect("Message has no value.");
    /// let value = bundle.format_pattern(&pattern, None, &mut errors);
    /// assert_eq!(&value, "5");
    /// ```
    ///
    /// [FTL syntax guide]: https://projectfluent.org/fluent/guide/functions.html
    pub fn add_function<F: 'static>(&mut self, id: &str, func: F) -> Result<(), FluentError>
    where
        F: for<'a> Fn(&[FluentValue<'a>], &FluentArgs) -> FluentValue<'a> + Sync + Send,
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
}

impl<R> Default for FluentBundle<R> {
    fn default() -> Self {
        let pr_langid = langid!("en");
        let langid = LanguageIdentifier::default();

        let pr = IntlPluralRules::create(pr_langid, PluralRuleType::CARDINAL)
            .expect("Failed to initialize PluralRules.");
        FluentBundle {
            plural_rules: pr,
            locales: vec![langid],
            resources: vec![],
            entries: Default::default(),
            use_isolating: false,
        }
    }
}
