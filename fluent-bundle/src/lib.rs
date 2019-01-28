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
//! use fluent_bundle::{FluentBundle, FluentValue, FluentResource};
//! use std::collections::HashMap;
//!
//! let ftl_string = String::from("
//! hello-world = Hello, world!
//! intro = Welcome, { $name }.
//! ");
//! let res = FluentResource::try_new(ftl_string)
//!     .expect("Could not parse an FTL string.");
//!
//! let mut bundle = FluentBundle::new(&["en-US"]);
//!
//! bundle
//!     .add_resource(&res)
//!     .expect("Failed to add FTL resources to the bundle.");
//!
//! let (value, _) = bundle
//!     .format("hello-world", None);
//!     .expect("Failed to format a message.");
//!
//! assert_eq!(&value, "Hello, world!");
//!
//! let mut args = HashMap::new();
//! args.insert("name", FluentValue::from("John"));
//!
//! let (value, _) = bundle
//!     .format("intro", Some(&args));
//!     .expect("Failed to format a message.");
//!
//! assert_eq!(value, "Welcome, John.");
//! ```

#[macro_use]
extern crate rental;
#[macro_use]
extern crate failure_derive;

pub mod bundle;
pub mod entry;
pub mod errors;
pub mod resolve;
pub mod resource;
pub mod types;

pub use bundle::FluentBundle;
pub use resource::FluentResource;
pub use types::FluentValue;
