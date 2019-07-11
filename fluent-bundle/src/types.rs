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

use std::borrow::{Borrow, Cow};
use std::fmt;
use std::str::FromStr;

use fluent_syntax::ast;
use intl_pluralrules::PluralCategory;

use crate::resolve::Scope;
use crate::resource::FluentResource;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum DisplayableNodeType {
    Message,
    Term,
    Variable,
    Function,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct DisplayableNode<'source> {
    node_type: DisplayableNodeType,
    id: &'source str,
    attribute: Option<&'source str>,
}

impl<'source> DisplayableNode<'source> {
    pub fn get_error(&self) -> String {
        match self.node_type {
            DisplayableNodeType::Message => format!("Unknown message: {}", self),
            DisplayableNodeType::Term => format!("Unknown term: {}", self),
            DisplayableNodeType::Variable => format!("Unknown variable: {}", self),
            DisplayableNodeType::Function => format!("Unknown function: {}", self),
        }
    }
}

impl<'source> fmt::Display for DisplayableNode<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.node_type {
            DisplayableNodeType::Message => write!(f, "{}", self.id)?,
            DisplayableNodeType::Term => write!(f, "-{}", self.id)?,
            DisplayableNodeType::Variable => write!(f, "${}", self.id)?,
            DisplayableNodeType::Function => write!(f, "{}()", self.id)?,
        };
        if let Some(attr) = self.attribute {
            write!(f, ".{}", attr)?;
        }
        Ok(())
    }
}

impl<'source> From<&ast::Message<'source>> for DisplayableNode<'source> {
    fn from(msg: &ast::Message<'source>) -> Self {
        DisplayableNode {
            node_type: DisplayableNodeType::Message,
            id: msg.id.name,
            attribute: None,
        }
    }
}

impl<'source> From<(&ast::Message<'source>, &ast::Attribute<'source>)>
    for DisplayableNode<'source>
{
    fn from(input: (&ast::Message<'source>, &ast::Attribute<'source>)) -> Self {
        DisplayableNode {
            node_type: DisplayableNodeType::Message,
            id: input.0.id.name,
            attribute: Some(input.1.id.name),
        }
    }
}

impl<'source> From<&ast::Term<'source>> for DisplayableNode<'source> {
    fn from(term: &ast::Term<'source>) -> Self {
        DisplayableNode {
            node_type: DisplayableNodeType::Term,
            id: term.id.name,
            attribute: None,
        }
    }
}

impl<'source> From<&ast::InlineExpression<'source>> for DisplayableNode<'source> {
    fn from(expr: &ast::InlineExpression<'source>) -> Self {
        match expr {
            ast::InlineExpression::MessageReference { id, ref attribute } => DisplayableNode {
                node_type: DisplayableNodeType::Message,
                id: id.name,
                attribute: attribute.as_ref().map(|attr| attr.name),
            },
            ast::InlineExpression::TermReference {
                id, ref attribute, ..
            } => DisplayableNode {
                node_type: DisplayableNodeType::Term,
                id: id.name,
                attribute: attribute.as_ref().map(|attr| attr.name),
            },
            ast::InlineExpression::VariableReference { id } => DisplayableNode {
                node_type: DisplayableNodeType::Variable,
                id: id.name,
                attribute: None,
            },
            ast::InlineExpression::FunctionReference { id, .. } => DisplayableNode {
                node_type: DisplayableNodeType::Function,
                id: id.name,
                attribute: None,
            },
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FluentValue<'source> {
    String(Cow<'source, str>),
    Number(Cow<'source, str>),
    None(),
    Error(DisplayableNode<'source>),
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
                let pr = &scope.bundle.plural_rules;
                pr.select(b.as_ref()) == Ok(cat)
            }
            _ => false,
        }
    }

    pub fn to_string(&self) -> Cow<'source, str> {
        match self {
            FluentValue::String(s) => s.clone(),
            FluentValue::Number(n) => n.clone(),
            FluentValue::Error(d) => d.to_string().into(),
            FluentValue::None() => "???".into(),
        }
    }
}

impl<'source> fmt::Display for FluentValue<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FluentValue::String(s) => write!(f, "{}", s),
            FluentValue::Number(n) => write!(f, "{}", n),
            FluentValue::Error(d) => write!(f, "{}", d),
            FluentValue::None() => write!(f, "???"),
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
