//! Fluent is a modern localization system designed to improve how software is translated.
//!
//! The Rust implementation provides the low level components for syntax operations, like parser
//! and AST, and the core localization struct - [`FluentBundle`].
//!
//! [`FluentBundle`] is the low level container for storing and formatting localization messages
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
//! let ftl_string = String::from("
//! hello-world = Hello, world!
//! intro = Welcome, { $name }.
//! ");
//! let res = FluentResource::try_new(ftl_string)
//!     .expect("Failed to parse an FTL string.");
//!
//! let langid_en = langid!("en-US");
//! let mut bundle = FluentBundle::new(&[langid_en]);
//!
//! bundle
//!     .add_resource(res)
//!     .expect("Failed to add FTL resources to the bundle.");
//!
//! let msg = bundle.get_message("hello-world")
//!     .expect("Message doesn't exist.");
//! let mut errors = vec![];
//! let pattern = msg.value
//!     .expect("Message has no value.");
//! let value = bundle.format_pattern(&pattern, None, &mut errors);
//!
//! assert_eq!(&value, "Hello, world!");
//!
//! let mut args = FluentArgs::new();
//! args.insert("name", FluentValue::from("John"));
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
//! [`FluentBundle`]: ./bundle/struct.FluentBundle.html
//! [`FluentResource`]: ./bundle/struct.FluentResource.html
//! [`FluentMessage`]: ./bundle/struct.FluentMessage.html
//! [`FluentArgs`]: ./bundle/type.FluentArgs.html
//! [`FluentValue`]: ./bundle/struct.FluentValue.html

#[macro_use]
extern crate rental;
#[macro_use]
extern crate failure_derive;

mod bundle;
mod entry;
mod errors;
pub mod resolve;
mod resource;
pub mod types;

pub use bundle::{FluentArgs, FluentBundle, FluentMessage};
pub use errors::FluentError;
pub use resource::FluentResource;
pub use types::FluentValue;
