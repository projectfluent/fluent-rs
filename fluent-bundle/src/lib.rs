//! Fluent is a modern localization system designed to improve how software is translated.
//!
//! `fluent-bundle` is the mid-level component of the [Fluent Localization
//! System](https://www.projectfluent.org).
//!
//! The crate builds on top of the low level [`fluent-syntax`](../fluent-syntax) package, and provides
//! foundational types and structures required for executing localization at runtime.
//!
//! There are four core concepts to understand Fluent runtime:
//!
//! * [`FluentMessage`] - A single translation unit
//! * [`FluentResource`] - A list of [`FluentMessage`] units
//! * [`FluentBundle`] - A collection of [`FluentResource`] lists
//! * [`FluentArgs`] - A list of [`FluentValue`] elements used to resolve a [`FluentMessage`] value
//!
//! # 1) Message
//!
//! [`FluentMessage`] is the core unit of the system.
//! It has an identifier, a value and a list of attributes.
//!
//! The identifier is a key that must be unique within a [`FluentResource`] to
//! which the message belongs to.
//!
//! The shape of the message must also contain a value, attributes or both.
//!
//! ### Simple Message
//!
//! ```text
//! hello-world = Hello, { $user }!
//! ```
//!
//! ### Compound Message
//!
//! ```text
//! confirm-modal = Are you sure?
//!     .confirm = Yes
//!     .cancel = No
//!     .tooltip = Closing the window will lose all unsaved data.
//! ```
//!
//! # 2) Resource
//!
//! [`FluentResource`] wraps an [`Abstract Syntax Tree`](../fluent_syntax/ast/index.html) produced by the
//! [`parser`](../fluent_syntax/parser/index.html) and provides an access to a list
//! of its entries.
//!
//! A good mental model for a resource is a single FTL file, but in the future
//! there's nothing preventing a resource from being stored in a data base,
//! pre-parsed format or in some other structured form.
//!
//! # 3) Bundle
//!
//! [`FluentBundle`] is the core structure of the Fluent system at runtime.
//!
//! It is a single-locale container storing a combination of multiple [`FluentResource`]
//! instances, combined with a set of internationalization components.
//!
//! The main function of the bundle is to provide a context in which messages can
//! reference each other, terms, and functions, and use memoized internationalization
//! components to provide resolved localization messages.
//!
//! A bundle is a thin wrapper, usually storing just references to the resources allocated
//! in a long-lived collection, like a resource manager or a simple vector.
//!
//! In result, [`FluentBundle`] is cheap to construct, and higher level APIs can
//! easily generate different permutations of [`FluentResource`] lists and
//! resolve messages within those combinations.
//!
//! # 4) Arguments & Values
//!
//! [`FluentArgs`] is a collection, similar to a `HashMap`, which stores a key-value pair list of
//! arguments provided by the developer to the [`format_pattern`](FluentBundle::format_pattern) method.
//! Those arguments are used during message formatting to resolve selections, or can be
//! interpolated into the message as a variable.
//!
//! The keys of the argument list are strings, and the values are limited to one of the
//! [`FluentValue`] types.
//!
//! # Summary
//!
//! Together, [`FluentMessage`], [`FluentResource`], [`FluentBundle`], and [`FluentArgs`] provide
//! all the necessary components of the Fluent localization system, and are sufficient
//! to complete a simple localization API.
//!
//! ## Example
//!
//! ```
//! use fluent_bundle::{FluentBundle, FluentValue, FluentResource, FluentArgs};
//!
//! // Used to provide a locale for the bundle.
//! use unic_langid::langid;
//!
//! let ftl_string = r#"
//!
//! hello-world = Hello, world!
//! intro = Welcome, { $name }.
//!
//! "#.to_string();
//!
//! let res = FluentResource::try_new(ftl_string)
//!     .expect("Failed to parse an FTL string.");
//!
//! let langid_en = langid!("en-US");
//! let mut bundle = FluentBundle::new(vec![langid_en]);
//!
//! bundle
//!     .add_resource(res)
//!     .expect("Failed to add FTL resources to the bundle.");
//!
//! let msg = bundle.get_message("hello-world")
//!     .expect("Message doesn't exist.");
//!
//! let mut errors = vec![];
//!
//! let pattern = msg.value()
//!     .expect("Message has no value.");
//!
//! let value = bundle.format_pattern(&pattern, None, &mut errors);
//!
//! assert_eq!(&value, "Hello, world!");
//!
//! let mut args = FluentArgs::new();
//! args.set("name", "John");
//!
//! let msg = bundle.get_message("intro")
//!     .expect("Message doesn't exist.");
//! let mut errors = vec![];
//! let pattern = msg.value().expect("Message has no value.");
//! let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);
//!
//! // The FSI/PDI isolation marks ensure that the direction of
//! // the text from the variable is not affected by the translation.
//! assert_eq!(value, "Welcome, \u{2068}John\u{2069}.");
//! ```
//!
//! # Ergonomics & Higher Level APIs
//!
//! Reading the example, you may notice how verbose it feels.
//! Many core methods are fallible, others accumulate errors, and there
//! are intermediate structures used in operations.
//!
//! This is intentional as it serves as building blocks for variety of different
//! scenarios allowing implementations to handle errors, cache and
//! optimize results.
//!
//! At the moment it is expected that users will use
//! the `fluent-bundle` crate directly, while the ecosystem
//! matures and higher level APIs are being developed.
//!
//! [`FluentBundle`]: ./type.FluentBundle.html
//! [`FluentResource`]: ./struct.FluentResource.html
//! [`FluentMessage`]: ./struct.FluentMessage.html
//! [`FluentValue`]: ./types/enum.FluentValue.html
//! [`FluentArgs`]: ./struct.FluentArgs.html
mod args;
mod bundle;
pub mod concurrent;
mod entry;
mod errors;
pub mod memoizer;
mod message;
pub mod resolver;
mod resource;
pub mod types;

pub use args::FluentArgs;
use bundle::FluentBundle as FluentBundleBase;

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
/// let pattern = msg.value().expect("Message has no value.");
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
pub type FluentBundle<R> = FluentBundleBase<R, intl_memoizer::IntlLangMemoizer>;
pub use errors::FluentError;
pub use message::{FluentAttribute, FluentMessage};
pub use resource::FluentResource;
#[doc(inline)]
pub use types::FluentValue;
