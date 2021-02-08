//! `FluentBundle` is a collection of localization messages in Fluent.
//!
//! It stores a list of messages in a single locale which can reference one another, use the same
//! internationalization formatters, functions, scopeironmental variables and are expected to be used
//! together.

use rustc_hash::FxHashMap;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::hash_map::Entry as HashEntry;
use std::default::Default;
use std::fmt;

use fluent_syntax::ast;
use unic_langid::LanguageIdentifier;

use crate::args::FluentArgs;
use crate::entry::Entry;
use crate::entry::GetEntry;
use crate::errors::{EntryKind, FluentError};
use crate::memoizer::MemoizerKind;
use crate::message::{FluentAttribute, FluentMessage};
use crate::resolver::{ResolveValue, Scope, WriteValue};
use crate::resource::FluentResource;
use crate::types::FluentValue;

/// A collection of localization messages for a single locale, which are meant
/// to be used together in a single view, widget or any other UI abstraction.
///
/// # Examples
///
/// ```
/// use fluent_bundle::{FluentBundle, FluentResource, FluentValue, FluentArgs};
/// use unic_langid::langid;
///
/// let ftl_string = String::from("intro = Welcome, { $name }.");
/// let resource = FluentResource::try_new(ftl_string)
///     .expect("Could not parse an FTL string.");
///
/// let langid_en = langid!("en-US");
/// let mut bundle = FluentBundle::new(vec![langid_en]);
///
/// bundle.add_resource(&resource)
///     .expect("Failed to add FTL resources to the bundle.");
///
/// let mut args = FluentArgs::new();
/// args.set("name", FluentValue::from("Rustacean"));
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
/// ## Create a bundle
///
/// To create a bundle, call [`FluentBundle::new`] with a locale list that represents the best
/// possible fallback chain for a given locale. The simplest case is a one-locale list.
///
/// Fluent uses [`LanguageIdentifier`] which can be created using `langid!` macro.
///
/// ## Add Resources
///
/// Next, call [`add_resource`] one or more times, supplying translations in the FTL syntax.
///
/// Since [`FluentBundle`] is generic over anything that can borrow a [`FluentResource`],
/// one can use [`FluentBundle`] to own its resources, store references to them,
/// or even [`Rc<FluentResource>`] or [`Arc<FluentResource>`].
///
/// The [`FluentBundle`] instance is now ready to be used for localization.
///
/// ## Format
///
/// To format a translation, call [`get_message`] to retrieve a [`FluentMessage`],
/// and then call [`format_pattern`] on the message value or attribute in order to
/// retrieve the translated string.
///
/// The result of [`format_pattern`] is an [`Cow<str>`]. It is
/// recommended to treat the result as opaque from the perspective of the program and use it only
/// to display localized messages. Do not examine it or alter in any way before displaying.  This
/// is a general good practice as far as all internationalization operations are concerned.
///
/// If errors were encountered during formatting, they will be
/// accumulated in the [`Vec<FluentError>`] passed as the third argument.
///
/// While they are not fatal, they usually indicate problems with the translation,
/// and should be logged or reported in a way that allows the developer to notice
/// and fix them.
///
///
/// # Locale Fallback Chain
///
/// [`FluentBundle`] stores messages in a single locale, but keeps a locale fallback chain for the
/// purpose of language negotiation with i18n formatters. For instance, if date and time formatting
/// are not available in the first locale, [`FluentBundle`] will use its `locales` fallback chain
/// to negotiate a sensible fallback for date and time formatting.
///
/// # Concurrency
///
/// As you may have noticed, `FluentBundle` is a specialization of [`FluentBundle`]
/// which works with an [`IntlMemoizer`][] over `RefCell`.
/// In scenarios where the memoizer must work concurrently, there's an implementation of
/// `IntlMemoizer` that uses `Mutex` and there's [`concurrent::FluentBundle`] which works with that.
///
/// [`add_resource`]: ./bundle/struct.FluentBundle.html#method.add_resource
/// [`FluentBundle::new`]: ./bundle/struct.FluentBundle.html#method.new
/// [`FluentMessage`]: ./struct.FluentMessage.html
/// [`FluentBundle`]: ./type.FluentBundle.html
/// [`FluentResource`]: ./struct.FluentResource.html
/// [`get_message`]: ./bundle/struct.FluentBundle.html#method.get_message
/// [`format_pattern`]: ./bundle/struct.FluentBundle.html#method.format_pattern
/// [`Cow<str>`]: http://doc.rust-lang.org/std/borrow/enum.Cow.html
/// [`Rc<FluentResource>`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
/// [`Arc<FluentResource>`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
/// [`LanguageIdentifier`]: https://crates.io/crates/unic-langid
/// [`IntlMemoizer`]: https://github.com/projectfluent/fluent-rs/tree/master/intl-memoizer
/// [`Vec<FluentError>`]: ./enum.FluentError.html
/// [`concurrent::FluentBundle`]: ./concurrent/type.FluentBundle.html
pub struct FluentBundle<R, M> {
    pub locales: Vec<LanguageIdentifier>,
    pub(crate) resources: Vec<R>,
    pub(crate) entries: FxHashMap<String, Entry>,
    pub(crate) intls: M,
    pub(crate) use_isolating: bool,
    pub(crate) transform: Option<fn(&str) -> Cow<str>>,
    pub(crate) formatter: Option<fn(&FluentValue, &M) -> Option<String>>,
}

impl<R, M> FluentBundle<R, M> {
    /// Adds a resource to the bundle, returning an empty [`Result<T>`] on success.
    ///
    /// If any entry in the resource uses the same identifier as an already
    /// existing key in the bundle, the new entry will be ignored and a
    /// `FluentError::Overriding` will be added to the result.
    ///
    /// The method can take any type that can be borrowed to `FluentResource`:
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
    /// let mut bundle = FluentBundle::new(vec![langid_en]);
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

        for (entry_pos, entry) in res.entries().enumerate() {
            let (id, entry, kind) = match entry {
                ast::Entry::Message(ast::Message { ref id, .. }) => (
                    id.name,
                    Entry::Message((res_pos, entry_pos)),
                    EntryKind::Message,
                ),
                ast::Entry::Term(ast::Term { ref id, .. }) => {
                    (id.name, Entry::Term((res_pos, entry_pos)), EntryKind::Term)
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

    /// Adds a resource to the bundle, returning an empty [`Result<T>`] on success.
    ///
    /// If any entry in the resource uses the same identifier as an already
    /// existing key in the bundle, the entry will override the previous one.
    ///
    /// The method can take any type that can be borrowed as FluentResource:
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
    ///
    /// let ftl_string = String::from("
    /// hello = Another Hi!
    /// ");
    /// let resource2 = FluentResource::try_new(ftl_string)
    ///     .expect("Could not parse an FTL string.");
    ///
    /// let langid_en = langid!("en-US");
    ///
    /// let mut bundle = FluentBundle::new(vec![langid_en]);
    /// bundle.add_resource(resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
    ///
    /// bundle.add_resource_overriding(resource2);
    ///
    /// let mut errors = vec![];
    /// let msg = bundle.get_message("hello")
    ///     .expect("Failed to retrieve the message");
    /// let value = msg.value.expect("Failed to retrieve the value of the message");
    /// assert_eq!(bundle.format_pattern(value, None, &mut errors), "Another Hi!");
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
    pub fn add_resource_overriding(&mut self, r: R)
    where
        R: Borrow<FluentResource>,
    {
        let res = r.borrow();
        let res_pos = self.resources.len();

        for (entry_pos, entry) in res.entries().enumerate() {
            let (id, entry) = match entry {
                ast::Entry::Message(ast::Message { ref id, .. }) => {
                    (id.name, Entry::Message((res_pos, entry_pos)))
                }
                ast::Entry::Term(ast::Term { ref id, .. }) => {
                    (id.name, Entry::Term((res_pos, entry_pos)))
                }
                _ => continue,
            };

            self.entries.insert(id.to_string(), entry);
        }
        self.resources.push(r);
    }

    /// When formatting patterns, `FluentBundle` inserts
    /// Unicode Directionality Isolation Marks to indicate
    /// that the direction of a placeable may differ from
    /// the surrounding message.
    ///
    /// This is important for cases such as when a
    /// right-to-left user name is presented in the
    /// left-to-right message.
    ///
    /// In some cases, such as testing, the user may want
    /// to disable the isolating.
    pub fn set_use_isolating(&mut self, value: bool) {
        self.use_isolating = value;
    }

    /// This method allows to specify a function that will
    /// be called on all textual fragments of the pattern
    /// during formatting.
    ///
    /// This is currently primarly used for pseudolocalization,
    /// and `fluent-pseudo` crate provides a function
    /// that can be passed here.
    pub fn set_transform(&mut self, func: Option<fn(&str) -> Cow<str>>) {
        if let Some(f) = func {
            self.transform = Some(f);
        } else {
            self.transform = None;
        }
    }

    /// This method allows to specify a function that will
    /// be called before any `FluentValue` is formatted
    /// allowing overrides.
    ///
    /// It's particularly useful for plugging in an external
    /// formatter for `FluentValue::Number`.
    pub fn set_formatter(&mut self, func: Option<fn(&FluentValue, &M) -> Option<String>>) {
        if let Some(f) = func {
            self.formatter = Some(f);
        } else {
            self.formatter = None;
        }
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
    /// let mut bundle = FluentBundle::new(vec![langid_en]);
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

    /// Retrieves a `FluentMessage` from a bundle.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_bundle::{FluentBundle, FluentResource};
    /// use unic_langid::langid;
    ///
    /// let ftl_string = String::from("hello-world = Hello World!");
    /// let resource = FluentResource::try_new(ftl_string)
    ///     .expect("Failed to parse an FTL string.");
    ///
    /// let langid_en = langid!("en-US");
    /// let mut bundle = FluentBundle::new(vec![langid_en]);
    ///
    /// bundle.add_resource(&resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
    ///
    /// let msg = bundle.get_message("hello-world");
    /// assert_eq!(msg.is_some(), true);
    /// ```
    pub fn get_message(&self, id: &str) -> Option<FluentMessage>
    where
        R: Borrow<FluentResource>,
    {
        let message = self.get_entry_message(id)?;
        let value = message.value.as_ref();
        let mut attributes = Vec::with_capacity(message.attributes.len());

        for attr in &message.attributes {
            attributes.push(FluentAttribute {
                id: attr.id.name,
                value: &attr.value,
            });
        }
        Some(FluentMessage { value, attributes })
    }

    /// Writes a formatted pattern which comes from a `FluentMessage`.
    ///
    /// # Example
    ///
    /// ```
    /// use fluent_bundle::{FluentBundle, FluentResource};
    /// use unic_langid::langid;
    ///
    /// let ftl_string = String::from("hello-world = Hello World!");
    /// let resource = FluentResource::try_new(ftl_string)
    ///     .expect("Failed to parse an FTL string.");
    ///
    /// let langid_en = langid!("en-US");
    /// let mut bundle = FluentBundle::new(vec![langid_en]);
    ///
    /// bundle.add_resource(&resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
    ///
    /// let msg = bundle.get_message("hello-world")
    ///     .expect("Failed to retrieve a FluentMessage.");
    ///
    /// let pattern = msg.value
    ///     .expect("Missing Value.");
    /// let mut errors = vec![];
    ///
    /// let mut s = String::new();
    /// bundle.write_pattern(&mut s, &pattern, None, &mut errors)
    ///     .expect("Failed to write.");
    ///
    /// assert_eq!(s, "Hello World!");
    /// ```
    pub fn write_pattern<'bundle, W>(
        &'bundle self,
        w: &mut W,
        pattern: &'bundle ast::Pattern<&str>,
        args: Option<&'bundle FluentArgs>,
        errors: &mut Vec<FluentError>,
    ) -> fmt::Result
    where
        R: Borrow<FluentResource>,
        W: fmt::Write,
        M: MemoizerKind,
    {
        let mut scope = Scope::new(self, args, Some(errors));
        pattern.write(w, &mut scope)
    }

    /// Formats a pattern which comes from a `FluentMessage`.
    ///
    /// # Example
    ///
    /// ```
    /// use fluent_bundle::{FluentBundle, FluentResource};
    /// use unic_langid::langid;
    ///
    /// let ftl_string = String::from("hello-world = Hello World!");
    /// let resource = FluentResource::try_new(ftl_string)
    ///     .expect("Failed to parse an FTL string.");
    ///
    /// let langid_en = langid!("en-US");
    /// let mut bundle = FluentBundle::new(vec![langid_en]);
    ///
    /// bundle.add_resource(&resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
    ///
    /// let msg = bundle.get_message("hello-world")
    ///     .expect("Failed to retrieve a FluentMessage.");
    ///
    /// let pattern = msg.value
    ///     .expect("Missing Value.");
    /// let mut errors = vec![];
    ///
    /// let result = bundle.format_pattern(&pattern, None, &mut errors);
    ///
    /// assert_eq!(result, "Hello World!");
    /// ```
    pub fn format_pattern<'bundle>(
        &'bundle self,
        pattern: &'bundle ast::Pattern<&str>,
        args: Option<&'bundle FluentArgs>,
        errors: &mut Vec<FluentError>,
    ) -> Cow<'bundle, str>
    where
        R: Borrow<FluentResource>,
        M: MemoizerKind,
    {
        let mut scope = Scope::new(self, args, Some(errors));
        let value = pattern.resolve(&mut scope);
        value.as_string(&scope)
    }

    /// Makes the provided rust function available to messages with the name `id`. See
    /// the [FTL syntax guide] to learn how these are used in messages.
    ///
    /// FTL functions accept both positional and named args. The rust function you
    /// provide therefore has two parameters: a slice of values for the positional
    /// args, and a `FluentArgs` for named args.
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
    /// let mut bundle = FluentBundle::new(vec![langid_en]);
    /// bundle.add_resource(&resource)
    ///     .expect("Failed to add FTL resources to the bundle.");
    ///
    /// // Register a fn that maps from string to string length
    /// bundle.add_function("STRLEN", |positional, _named| match positional {
    ///     [FluentValue::String(str)] => str.len().into(),
    ///     _ => FluentValue::Error,
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
    pub fn add_function<F>(&mut self, id: &str, func: F) -> Result<(), FluentError>
    where
        F: for<'a> Fn(&[FluentValue<'a>], &FluentArgs) -> FluentValue<'a> + Sync + Send + 'static,
    {
        match self.entries.entry(id.to_owned()) {
            HashEntry::Vacant(entry) => {
                entry.insert(Entry::Function(Box::new(func)));
                Ok(())
            }
            HashEntry::Occupied(_) => Err(FluentError::Overriding {
                kind: EntryKind::Function,
                id: id.to_owned(),
            }),
        }
    }
}

impl<R> Default for FluentBundle<R, intl_memoizer::IntlLangMemoizer> {
    fn default() -> Self {
        Self::new(vec![LanguageIdentifier::default()])
    }
}

impl<R> FluentBundle<R, intl_memoizer::IntlLangMemoizer> {
    /// Constructs a FluentBundle. The first element in `locales` should be the
    /// language this bundle represents, and will be used to determine the
    /// correct plural rules for this bundle. You can optionally provide extra
    /// languages in the list; they will be used as fallback date and time
    /// formatters if a formatter for the primary language is unavailable.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_bundle::FluentBundle;
    /// use fluent_bundle::FluentResource;
    /// use unic_langid::langid;
    ///
    /// let langid_en = langid!("en-US");
    /// let mut bundle: FluentBundle<FluentResource, _> = FluentBundle::new(vec![langid_en]);
    /// ```
    ///
    /// # Errors
    ///
    /// This will panic if no formatters can be found for the locales.
    pub fn new(locales: Vec<LanguageIdentifier>) -> Self {
        let first_locale = locales.get(0).cloned().unwrap_or_default();
        Self {
            locales,
            resources: vec![],
            entries: FxHashMap::default(),
            intls: intl_memoizer::IntlLangMemoizer::new(first_locale),
            use_isolating: true,
            transform: None,
            formatter: None,
        }
    }
}

impl crate::memoizer::MemoizerKind for intl_memoizer::IntlLangMemoizer {
    fn new(lang: LanguageIdentifier) -> Self
    where
        Self: Sized,
    {
        Self::new(lang)
    }

    fn with_try_get_threadsafe<I, R, U>(&self, args: I::Args, cb: U) -> Result<R, I::Error>
    where
        Self: Sized,
        I: intl_memoizer::Memoizable + Send + Sync + 'static,
        I::Args: Send + Sync + 'static,
        U: FnOnce(&I) -> R,
    {
        self.with_try_get(args, cb)
    }

    fn stringify_value(
        &self,
        value: &dyn crate::types::FluentType,
    ) -> std::borrow::Cow<'static, str> {
        value.as_string(self)
    }
}
