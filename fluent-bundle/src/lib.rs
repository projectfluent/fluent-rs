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
//! * [`FluentBundle`](crate::bundle::FluentBundle) - A collection of [`FluentResource`] lists
//! * [`FluentArgs`] - A list of elements used to resolve a [`FluentMessage`] value
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
//! arguments provided by the developer to the [`format_pattern`](crate::bundle::FluentBundle::format_pattern) method.
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
pub mod bundle;
mod concurrent;
mod entry;
mod errors;
pub mod memoizer;
mod message;
pub mod resolver;
mod resource;
pub mod types;

pub use args::FluentArgs;
/// Specialized [`FluentBundle`](crate::bundle::FluentBundle) over
/// non-concurrent [`IntlLangMemoizer`](intl_memoizer::IntlLangMemoizer).
///
/// This is the basic variant of the [`FluentBundle`](crate::bundle::FluentBundle).
///
/// The concurrent specialization, can be constructed with
/// [`FluentBundle::new_concurrent`](crate::bundle::FluentBundle::new_concurrent).
pub type FluentBundle<R> = bundle::FluentBundle<R, intl_memoizer::IntlLangMemoizer>;
pub use errors::FluentError;
pub use message::{FluentAttribute, FluentMessage};
pub use resource::FluentResource;
#[doc(inline)]
pub use types::FluentValue;
