#![feature(box_patterns)]

//! Fluent is a localization system designed to improve how software is translated.
//!
//! The Rust implementation provides the low level components for syntax operations, like parser
//! and AST, and the core localization struct - `MessageContext`.
//!
//! `MessageContext` is the low level container for storing and formating localization messages. It
//! is expected that implementations will build on top of it by providing language negotiation
//! between user requested languages and available resources and I/O for loading selected
//! resources.
//!
//! # Example
//!
//! ```
//! use fluent::MessageContext;
//! use fluent::types::FluentValue;
//! use std::collections::HashMap;
//!
//! let mut ctx = MessageContext::new(&["en-US"]);
//!
//! ctx.add_messages(
//!     "
//! hello-world = Hello, world!
//! intro = Welcome, { $name }.
//! "
//!     );
//!
//! let msg = ctx.get_message("hello-world").unwrap();
//! let value = ctx.format(msg, None).unwrap();
//!
//! assert_eq!(value, "Hello, world!");
//!
//! let mut args = HashMap::new();
//! args.insert("name", FluentValue::from("John"));
//!
//! let msg = ctx.get_message("intro").unwrap();
//! let value = ctx.format(msg, Some(&args)).unwrap();
//!
//! assert_eq!(value, "Welcome, John.");
//! ```

pub mod syntax;
pub mod context;
pub mod resolve;
pub mod types;
pub mod intl;

pub use context::MessageContext;
