//! Fluent is a modern localization system designed to improve how software is translated.
//!
//! `fluent-bundle` is the mid-level component of the [Fluent Localization
//! System](https://www.projectfluent.org).
//!
//! It builds on top of the low level [`fluent-syntax`](../fluent-syntax) package, and provides
//! foundational types and structures required for executing localization at runtime.
//!
//! # Fluent Bundle
//! The core structure on that level is [`FluentBundle`].
//!
//! [`FluentBundle`] is a low level container for storing and formatting localization messages
//! in a single locale.
//!
//! This crate provides also a number of structures needed for a localization API such as [`FluentResource`],
//! [`FluentMessage`], [`FluentArgs`], and [`FluentValue`].
//!
//! Together, they allow implementations to build higher-level APIs that use [`FluentBundle`]
//! and add user friendly helpers, framework bindings, error fallbacking,
//! language negotiation between user requested languages and available resources,
//! and I/O for loading selected resources.
//!
//! # Example
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
//! let pattern = msg.value
//!     .expect("Message has no value.");
//!
//! let value = bundle.format_pattern(&pattern, None, &mut errors);
//!
//! assert_eq!(&value, "Hello, world!");
//!
//! let mut args = FluentArgs::new();
//! args.add("name", "John");
//!
//! let msg = bundle.get_message("intro")
//!     .expect("Message doesn't exist.");
//! let mut errors = vec![];
//! let pattern = msg.value.expect("Message has no value.");
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
#[doc(inline)]
pub use bundle::FluentBundle;
pub use errors::FluentError;
pub use message::{FluentAttribute, FluentMessage};
pub use resource::FluentResource;
#[doc(inline)]
pub use types::FluentValue;
