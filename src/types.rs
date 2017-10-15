//! The `FluentValue` enum represents values which can be formatted to a String.
//!
//! The [`ResolveValue`][] trait from the [`resolve`][] module evaluates AST nodes into
//! `FluentValues` which can then be formatted to Strings using the i18n formatters stored by the
//! `MessageContext` instance if required.
//!
//! The arguments `HashMap` passed to [`MessageContext::format`][] should also use `FluentValues`
//! as values of arguments.
//!
//! [`ResolveValue`]: ../resolve/trait.ResolveValue.html
//! [`resolve`]: ../resolve
//! [`MessageContext::format`]: ../context/struct.MessageContext.html#method.format

use std::f32;

use super::context::MessageContext;
use super::intl::PluralRules;

/// Value types which can be formatted to a String.
#[derive(Clone, Debug, PartialEq)]
pub enum FluentValue {
    /// Fluent String type.
    String(String),
    /// Fluent Number type.
    Number(f32),
}

impl FluentValue {
    pub fn format(&self, _ctx: &MessageContext) -> String {
        match *self {
            FluentValue::String(ref s) => s.clone(),
            FluentValue::Number(ref n) => format!("{}", n),
        }
    }

    pub fn matches(&self, ctx: &MessageContext, other: &FluentValue) -> bool {
        match (self, other) {
            (&FluentValue::String(ref a), &FluentValue::String(ref b)) => a == b,
            (&FluentValue::Number(ref a), &FluentValue::Number(ref b)) => {
                (a - b).abs() < f32::EPSILON
            }
            (&FluentValue::String(ref a), &FluentValue::Number(ref b)) => {
                //XXX: This is a dirty hack and should be replaced with a
                //lazy resolved cache on the context.
                let pr = PluralRules::new(ctx.locales);
                pr.select(*b) == a
            }
            (&FluentValue::Number(..), &FluentValue::String(..)) => false,
        }
    }
}

impl From<String> for FluentValue {
    fn from(s: String) -> Self {
        FluentValue::String(s)
    }
}

impl<'a> From<&'a str> for FluentValue {
    fn from(s: &'a str) -> Self {
        FluentValue::String(String::from(s))
    }
}

impl From<f32> for FluentValue {
    fn from(n: f32) -> Self {
        FluentValue::Number(n)
    }
}

impl From<i8> for FluentValue {
    fn from(n: i8) -> Self {
        FluentValue::Number(f32::from(n))
    }
}
