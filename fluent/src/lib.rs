//! Fluent is a localization system designed to improve how software is translated.
//!
//! The Rust implementation provides the low level components for syntax operations, like parser
//! and AST, and the core localization struct - `FluentBundle`.
//!
//! `FluentBundle` is the low level container for storing and formatting localization messages. It
//! is expected that implementations will build on top of it by providing language negotiation
//! between user requested languages and available resources and I/O for loading selected
//! resources.
//!
//! # Example
//!
//! ```
//! use fluent::FluentBundle;
//! use fluent::types::FluentValue;
//! use std::collections::HashMap;
//!
//! let mut bundle = FluentBundle::new(&["en-US"]);
//! bundle.add_messages(
//!     "
//! hello-world = Hello, world!
//! intro = Welcome, { $name }.
//! "
//!     );
//!
//! let value = bundle.format("hello-world", None).unwrap();
//! assert_eq!(value, "Hello, world!");
//!
//! let mut args = HashMap::new();
//! args.insert("name", FluentValue::from("John"));
//!
//! let value = bundle.format("intro", Some(&args)).unwrap();
//! assert_eq!(value, "Welcome, John.");
//! ```

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate fluent_locale;
extern crate fluent_syntax;
extern crate intl_pluralrules;

pub mod context;
pub mod entry;
pub mod errors;
pub mod resolve;
pub mod types;

pub use context::FluentBundle;
