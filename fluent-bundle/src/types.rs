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

use std::borrow::Cow;
use std::str::FromStr;

use intl_pluralrules::PluralCategory;

use super::resolve::Env;

#[derive(Debug, PartialEq)]
pub enum FluentValueError {
    ParseError,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FluentValue<'v> {
    String(Cow<'v, str>),
    Number(Cow<'v, str>),
    None(Option<Cow<'v, str>>),
}

impl<'v> FluentValue<'v> {
    pub fn into_number<S: ToString>(v: S) -> Self {
        match f64::from_str(&v.to_string()) {
            Ok(_) => FluentValue::Number(v.to_string().into()),
            Err(_) => FluentValue::String(v.to_string().into()),
        }
    }

    pub fn matches(&self, other: &FluentValue, env: &Env) -> bool {
        match (self, other) {
            (&FluentValue::String(ref a), &FluentValue::String(ref b)) => a == b,
            (&FluentValue::Number(ref a), &FluentValue::Number(ref b)) => a == b,
            (&FluentValue::String(ref a), &FluentValue::Number(ref b)) => {
                let cat = match a.as_ref() {
                    "zero" => PluralCategory::ZERO,
                    "one" => PluralCategory::ONE,
                    "two" => PluralCategory::TWO,
                    "few" => PluralCategory::FEW,
                    "many" => PluralCategory::MANY,
                    "other" => PluralCategory::OTHER,
                    _ => return false,
                };
                let pr = &env.bundle.plural_rules;
                pr.select(b.as_ref()) == Ok(cat)
            }
            _ => false,
        }
    }

    pub fn to_string(&self) -> Cow<'v, str> {
        match self {
            FluentValue::String(s) => s.clone(),
            FluentValue::Number(n) => n.clone(),
            FluentValue::None(fallback) => {
                if let Some(fallback) = fallback {
                    fallback.clone()
                } else {
                    String::from("???").into()
                }
            }
        }
    }
}

impl<'v> From<String> for FluentValue<'v> {
    fn from(s: String) -> Self {
        FluentValue::String(s.into())
    }
}

impl<'v> From<&'v str> for FluentValue<'v> {
    fn from(s: &'v str) -> Self {
        FluentValue::String(s.into())
    }
}

impl<'v> From<f64> for FluentValue<'v> {
    fn from(n: f64) -> Self {
        FluentValue::Number(n.to_string().into())
    }
}

impl<'v> From<isize> for FluentValue<'v> {
    fn from(n: isize) -> Self {
        FluentValue::Number(n.to_string().into())
    }
}
