use std::f32;
use super::resolve::Env;

#[derive(Clone, Debug, PartialEq)]
pub enum FluentValue {
    String(String),
    Number(f32),
}

// XXX Replace this with a proper plural rule based on the MessageContext's locales.
fn mock_plural(num: &f32) -> &'static str {
    if (*num - 1.0).abs() < f32::EPSILON {
        "one"
    } else {
        "other"
    }
}

impl FluentValue {
    pub fn format(&self) -> String {
        match *self {
            FluentValue::String(ref s) => s.clone(),
            FluentValue::Number(ref n) => format!("{}", n),
        }
    }

    pub fn matches(&self, _env: &Env, other: &FluentValue) -> bool {
        match (self, other) {
            (&FluentValue::String(ref a), &FluentValue::String(ref b)) => a == b,
            (&FluentValue::Number(ref a), &FluentValue::Number(ref b)) => {
                (a - b).abs() < f32::EPSILON
            }
            (&FluentValue::String(ref a), &FluentValue::Number(ref b)) => a == mock_plural(b),
            (&FluentValue::Number(..), &FluentValue::String(..)) => false,
        }
    }
}

impl From<String> for FluentValue {
    fn from(s: String) -> Self {
        FluentValue::String(s)
    }
}

impl From<&'static str> for FluentValue {
    fn from(s: &'static str) -> Self {
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
