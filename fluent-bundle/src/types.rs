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

use std::any::Any;
use std::borrow::{Borrow, Cow};
use std::default::Default;
use std::fmt;
use std::str::FromStr;

use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use fluent_syntax::ast;
use intl_memoizer::Memoizable;
use intl_pluralrules::{PluralCategory, PluralRuleType, PluralRules as IntlPluralRules};
use unic_langid::LanguageIdentifier;

use crate::resolve::Scope;
use crate::resource::FluentResource;

struct PluralRules(pub IntlPluralRules);

impl Memoizable for PluralRules {
    type Args = (PluralRuleType,);
    type Error = &'static str;
    fn construct(lang: LanguageIdentifier, args: Self::Args) -> Result<Self, Self::Error> {
        let default_lang: LanguageIdentifier = "en".parse().unwrap();
        let pr_lang = negotiate_languages(
            &[lang],
            &IntlPluralRules::get_locales(args.0),
            Some(&default_lang),
            NegotiationStrategy::Lookup,
        )[0]
        .clone();
        Ok(Self(IntlPluralRules::create(pr_lang, args.0)?))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DisplayableNodeType<'source> {
    Message(&'source str),
    Term(&'source str),
    Variable(&'source str),
    Function(&'source str),
    Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DisplayableNode<'source> {
    node_type: DisplayableNodeType<'source>,
    attribute: Option<&'source str>,
}

impl<'source> Default for DisplayableNode<'source> {
    fn default() -> Self {
        DisplayableNode {
            node_type: DisplayableNodeType::Expression,
            attribute: None,
        }
    }
}

impl<'source> DisplayableNode<'source> {
    pub fn get_error(&self) -> String {
        if self.attribute.is_some() {
            format!("Unknown attribute: {}", self)
        } else {
            match self.node_type {
                DisplayableNodeType::Message(..) => format!("Unknown message: {}", self),
                DisplayableNodeType::Term(..) => format!("Unknown term: {}", self),
                DisplayableNodeType::Variable(..) => format!("Unknown variable: {}", self),
                DisplayableNodeType::Function(..) => format!("Unknown function: {}", self),
                DisplayableNodeType::Expression => "Failed to resolve an expression.".to_string(),
            }
        }
    }
}

impl<'source> fmt::Display for DisplayableNode<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.node_type {
            DisplayableNodeType::Message(id) => write!(f, "{}", id)?,
            DisplayableNodeType::Term(id) => write!(f, "-{}", id)?,
            DisplayableNodeType::Variable(id) => write!(f, "${}", id)?,
            DisplayableNodeType::Function(id) => write!(f, "{}()", id)?,
            DisplayableNodeType::Expression => f.write_str("???")?,
        };
        if let Some(attr) = self.attribute {
            write!(f, ".{}", attr)?;
        }
        Ok(())
    }
}

impl<'source> From<&ast::Expression<'source>> for DisplayableNode<'source> {
    fn from(expr: &ast::Expression<'source>) -> Self {
        match expr {
            ast::Expression::InlineExpression(e) => e.into(),
            ast::Expression::SelectExpression { .. } => DisplayableNode::default(),
        }
    }
}

impl<'source> From<&ast::InlineExpression<'source>> for DisplayableNode<'source> {
    fn from(expr: &ast::InlineExpression<'source>) -> Self {
        match expr {
            ast::InlineExpression::MessageReference { id, attribute } => DisplayableNode {
                node_type: DisplayableNodeType::Message(id.name),
                attribute: attribute.as_ref().map(|attr| attr.name),
            },
            ast::InlineExpression::TermReference { id, attribute, .. } => DisplayableNode {
                node_type: DisplayableNodeType::Term(id.name),
                attribute: attribute.as_ref().map(|attr| attr.name),
            },
            ast::InlineExpression::VariableReference { id } => DisplayableNode {
                node_type: DisplayableNodeType::Variable(id.name),
                attribute: None,
            },
            ast::InlineExpression::FunctionReference { id, .. } => DisplayableNode {
                node_type: DisplayableNodeType::Function(id.name),
                attribute: None,
            },
            _ => DisplayableNode::default(),
        }
    }
}

pub trait FluentType: fmt::Debug + fmt::Display + AnyEq + 'static {
    fn duplicate(&self) -> Box<dyn FluentType>;
    fn as_string(&self) -> Cow<'static, str>;
}

impl PartialEq for dyn FluentType {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other.as_any())
    }
}

pub trait AnyEq: Any + 'static {
    fn equals(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any + PartialEq> AnyEq for T {
    fn equals(&self, other: &dyn Any) -> bool {
        if let Some(that) = other.downcast_ref::<Self>() {
            self == that
        } else {
            false
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub enum FluentValue<'source> {
    String(Cow<'source, str>),
    Number(Cow<'source, str>),
    Custom(Box<dyn FluentType>),
    Error(DisplayableNode<'source>),
    None,
}

impl<'s> PartialEq for FluentValue<'s> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FluentValue::String(s), FluentValue::String(s2)) => s == s2,
            (FluentValue::Number(s), FluentValue::Number(s2)) => s == s2,
            (FluentValue::Custom(s), FluentValue::Custom(s2)) => s == s2,
            _ => false,
        }
    }
}

impl<'s> Clone for FluentValue<'s> {
    fn clone(&self) -> Self {
        match self {
            FluentValue::String(s) => FluentValue::String(s.clone()),
            FluentValue::Number(s) => FluentValue::Number(s.clone()),
            FluentValue::Custom(s) => {
                let new_value: Box<dyn FluentType> = s.duplicate();
                FluentValue::Custom(new_value)
            }
            _ => panic!(),
        }
    }
}

impl<'source> FluentValue<'source> {
    pub fn into_number<S: ToString>(v: S) -> Self {
        let s = v.to_string();
        match f64::from_str(&s) {
            Ok(_) => FluentValue::Number(s.into()),
            Err(_) => FluentValue::String(s.into()),
        }
    }

    pub fn matches<R: Borrow<FluentResource>>(
        &self,
        other: &FluentValue,
        scope: &Scope<R>,
    ) -> bool {
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
                let mut intls_borrow = scope.bundle.intls.borrow_mut();
                let pr = intls_borrow
                    .try_get::<PluralRules>((PluralRuleType::CARDINAL,))
                    .expect("Failed to retrieve plural rules");
                pr.0.select(b.as_ref()) == Ok(cat)
            }
            _ => false,
        }
    }

    pub fn as_string(&self) -> Cow<'source, str> {
        match self {
            FluentValue::String(s) => s.clone(),
            FluentValue::Number(n) => n.clone(),
            FluentValue::Error(d) => format!("{{{}}}", d.to_string()).into(),
            FluentValue::Custom(s) => s.as_string(),
            FluentValue::None => "???".into(),
        }
    }
}

impl<'source> fmt::Display for FluentValue<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FluentValue::String(s) => f.write_str(s),
            FluentValue::Number(n) => f.write_str(n),
            FluentValue::Error(d) => write!(f, "{{{}}}", d),
            FluentValue::Custom(s) => s.fmt(f),
            FluentValue::None => f.write_str("???"),
        }
    }
}

impl<'source> From<String> for FluentValue<'source> {
    fn from(s: String) -> Self {
        FluentValue::String(s.into())
    }
}

impl<'source> From<&'source str> for FluentValue<'source> {
    fn from(s: &'source str) -> Self {
        FluentValue::String(s.into())
    }
}

macro_rules! from_num {
    ($num:ty) => {
        impl<'source> From<$num> for FluentValue<'source> {
            fn from(n: $num) -> Self {
                FluentValue::Number(n.to_string().into())
            }
        }
        impl<'source> From<&'source $num> for FluentValue<'source> {
            fn from(n: &'source $num) -> Self {
                FluentValue::Number(n.to_string().into())
            }
        }
    };
}
from_num!(i8);
from_num!(i16);
from_num!(i32);
from_num!(i64);
from_num!(i128);
from_num!(isize);
from_num!(u8);
from_num!(u16);
from_num!(u32);
from_num!(u64);
from_num!(u128);
from_num!(usize);
from_num!(f32);
from_num!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_from_copy_ref() {
        let x = 1i16;
        let y = &x;
        let z: FluentValue = y.into();
        assert_eq!(z, FluentValue::Number("1".into()));
    }
}
