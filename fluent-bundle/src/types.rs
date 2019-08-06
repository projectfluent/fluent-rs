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
use std::default::Default;
use std::fmt;
use std::str::FromStr;

use fluent_syntax::ast;
use intl_pluralrules::PluralCategory;

use crate::resolve::Scope;
use crate::resource::FluentResource;

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

#[derive(Debug, PartialEq, Clone)]
pub enum FluentValue<'source> {
    String(Cow<'source, str>),
    Number(Cow<'source, str>),
    Error(DisplayableNode<'source>),
    None,
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
            FluentValue::Error(d) => format!("{{{}}}", d.to_string()).into(),
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
