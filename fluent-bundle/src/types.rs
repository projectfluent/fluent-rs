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

use super::resolve::Scope;
use fluent_syntax::ast;

#[derive(Debug, PartialEq, Clone)]
pub enum DisplayableNodeType {
    Message,
    Term,
    Variable,
    Function,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DisplayableNode<'v> {
    pub node_type: DisplayableNodeType,
    pub id: &'v str,
    pub attribute: Option<&'v str>,
}

impl<'v> DisplayableNode<'v> {
    pub fn display(&self) -> String {
        let mut id = self.id.to_owned();
        if let Some(attr) = self.attribute {
            id.push_str(".");
            id.push_str(attr);
        }
        match self.node_type {
            DisplayableNodeType::Message => id,
            DisplayableNodeType::Term => format!("-{}", id),
            DisplayableNodeType::Variable => format!("${}", id),
            DisplayableNodeType::Function => format!("{}()", id),
        }
    }

    pub fn get_error(&self) -> String {
        let mut id = match self.node_type {
            DisplayableNodeType::Message => String::from("Unknown message: "),
            DisplayableNodeType::Term => String::from("Unknown term: "),
            DisplayableNodeType::Variable => String::from("Unknown variable: "),
            DisplayableNodeType::Function => String::from("Unknown function: "),
        };
        id.push_str(&self.display());
        id
    }

    pub fn new(id: &'v str, attribute: Option<&'v str>) -> Self {
        DisplayableNode {
            node_type: DisplayableNodeType::Message,
            id,
            attribute,
        }
    }
}

impl<'v> From<&ast::Message<'v>> for DisplayableNode<'v> {
    fn from(msg: &ast::Message<'v>) -> Self {
        DisplayableNode {
            node_type: DisplayableNodeType::Message,
            id: msg.id.name,
            attribute: None,
        }
    }
}

impl<'v> From<&ast::Term<'v>> for DisplayableNode<'v> {
    fn from(term: &ast::Term<'v>) -> Self {
        DisplayableNode {
            node_type: DisplayableNodeType::Term,
            id: term.id.name,
            attribute: None,
        }
    }
}

impl<'v> From<&ast::InlineExpression<'v>> for DisplayableNode<'v> {
    fn from(expr: &ast::InlineExpression<'v>) -> Self {
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
pub enum FluentValue<'v> {
    String(Cow<'v, str>),
    Number(Cow<'v, str>),
    None(),
    Error(DisplayableNode<'v>),
}

impl<'v> FluentValue<'v> {
    pub fn into_number<S: ToString>(v: S) -> Self {
        match f64::from_str(&v.to_string()) {
            Ok(_) => FluentValue::Number(v.to_string().into()),
            Err(_) => FluentValue::String(v.to_string().into()),
        }
    }

    pub fn matches(&self, other: &FluentValue, env: &Scope) -> bool {
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
            FluentValue::Error(d) => d.display().into(),
            FluentValue::None() => String::from("???").into(),
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
