//! The `FluentValue` enum represents values which can be formatted to a String.
//!
//! The [`ResolveValue`][] trait from the [`resolve`][] module evaluates AST nodes into
//! `FluentValues` which can then be formatted to Strings using the i18n formatters stored by the
//! `FluentBundle` instance if required.
//!
//! The arguments `HashMap` passed to [`FluentBundle::format`][] should also use `FluentValues`
//! as values of arguments.
//!
//! [`ResolveValue`]: ../resolve/trait.ResolveValue.html
//! [`resolve`]: ../resolve
//! [`FluentBundle::format`]: ../bundle/struct.FluentBundle.html#method.format

use std::f32;
use std::num::ParseFloatError;
use std::str::FromStr;

use intl_pluralrules::PluralCategory;

use super::bundle::FluentBundle;

/// Value types which can be formatted to a String.
#[derive(Clone, Debug, PartialEq)]
pub enum FluentValue {
    /// Fluent String type.
    String(String),
    /// Fluent Number type.
    Number(String),
}

impl FluentValue {
    pub fn as_number<S: ToString>(v: S) -> Result<Self, ParseFloatError> {
        f64::from_str(&v.to_string()).map(|_| FluentValue::Number(v.to_string()))
    }

    pub fn format(&self, _bundle: &FluentBundle) -> String {
        match self {
            FluentValue::String(s) => s.clone(),
            FluentValue::Number(n) => n.clone(),
        }
    }

    pub fn matches(&self, bundle: &FluentBundle, other: &FluentValue) -> bool {
        match (self, other) {
            (&FluentValue::String(ref a), &FluentValue::String(ref b)) => a == b,
            (&FluentValue::Number(ref a), &FluentValue::Number(ref b)) => a == b,
            (&FluentValue::String(ref a), &FluentValue::Number(ref b)) => {
                let cat = match a.as_str() {
                    "zero" => PluralCategory::ZERO,
                    "one" => PluralCategory::ONE,
                    "two" => PluralCategory::TWO,
                    "few" => PluralCategory::FEW,
                    "many" => PluralCategory::MANY,
                    "other" => PluralCategory::OTHER,
                    _ => return false,
                };

                let pr = &bundle.plural_rules;
                pr.select(b.as_str()) == Ok(cat)
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
        FluentValue::Number(n.to_string())
    }
}

impl From<isize> for FluentValue {
    fn from(n: isize) -> Self {
        FluentValue::Number(n.to_string())
    }
}
