//! AST, parser and serializer operations
//!
//! This is an internal API used by `MessageContext` for parsing an FTL syntax
//! into an AST that can be then resolved by the `Resolver`.
//!
//! This module may be useful for tooling that operates on FTL syntax.

#[macro_use]
pub mod errors;
pub mod ast;
pub mod parser;
pub mod stream;
pub mod ftlstream;

pub use self::parser::parse;
