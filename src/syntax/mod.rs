
pub use self::parser::parse;

pub mod ast;
pub mod parser;

#[cfg(all(feature = "json", not(feature = "entries-json")))]
pub mod json;

#[cfg(all(feature = "entries-json", not(feature = "json")))]
pub mod runtime;
#[cfg(all(feature = "entries-json", not(feature = "json")))]
pub use self::runtime::json;

