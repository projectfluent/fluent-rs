//! Abstract Syntax Tree representation of the Fluent Translation List.
//!
//! The AST of Fluent contains all nodes structures to represent a complete
//! representation of the FTL resource.
//!
//! The AST is intended to preserve all semantic information and allow for round-trip
//! of a canonically written FTL resource.
//!
//! # Example
//!
//! ```
//! use fluent_syntax::parser;
//! use fluent_syntax::ast;
//!
//! let ftl = r#"
//!
//! ## This is a message comment
//! hello-world = Hello World!
//!     .tooltip = Tooltip for you, { $userName }.
//!
//! "#;
//!
//! let resource = parser::parse(ftl)
//!     .expect("Failed to parse an FTL resource.");
//!
//! assert_eq!(
//!     resource.body[0],
//!     ast::Entry::Message(
//!         ast::Message {
//!             id: ast::Identifier {
//!                 name: "hello-world"
//!             },
//!             value: Some(ast::Pattern {
//!                 elements: vec![
//!                     ast::PatternElement::TextElement {
//!                         value: "Hello World!"
//!                     },
//!                 ]
//!             }),
//!             attributes: vec![
//!                 ast::Attribute {
//!                     id: ast::Identifier {
//!                         name: "tooltip"
//!                     },
//!                     value: ast::Pattern {
//!                         elements: vec![
//!                             ast::PatternElement::TextElement {
//!                                 value: "Tooltip for you, "
//!                             },
//!                             ast::PatternElement::Placeable {
//!                                 expression: ast::Expression::InlineExpression(
//!                                     ast::InlineExpression::VariableReference {
//!                                         id: ast::Identifier {
//!                                             name: "userName"
//!                                         }
//!                                     }
//!                                 )
//!                             },
//!                             ast::PatternElement::TextElement {
//!                                 value: "."
//!                             },
//!                         ]
//!                     }
//!                 }
//!             ],
//!             comment: Some(
//!                 ast::Comment {
//!                     content: vec!["This is a message comment"]
//!                 }
//!             )
//!         }
//!     ),
//! );
//! ```
//!
//! ## Errors
//!
//! AST does preserve blocks containing invaid syntax as [`Entry::Junk`].
//!
//! ## White space
//!
//! At the moment, AST does not preserve white space. In result only a
//! canonical form of the AST is suitable for a round-trip.
mod helper;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Resource<S> {
    pub body: Vec<Entry<S>>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum Entry<S> {
    Message(Message<S>),
    Term(Term<S>),
    Comment(Comment<S>),
    GroupComment(Comment<S>),
    ResourceComment(Comment<S>),
    Junk { content: S },
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Message<S> {
    pub id: Identifier<S>,
    pub value: Option<Pattern<S>>,
    pub attributes: Vec<Attribute<S>>,
    pub comment: Option<Comment<S>>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Term<S> {
    pub id: Identifier<S>,
    pub value: Pattern<S>,
    pub attributes: Vec<Attribute<S>>,
    pub comment: Option<Comment<S>>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pattern<S> {
    pub elements: Vec<PatternElement<S>>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum PatternElement<S> {
    TextElement { value: S },
    Placeable { expression: Expression<S> },
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Attribute<S> {
    pub id: Identifier<S>,
    pub value: Pattern<S>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Identifier<S> {
    pub name: S,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub struct Variant<S> {
    pub key: VariantKey<S>,
    pub value: Pattern<S>,
    pub default: bool,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum VariantKey<S> {
    Identifier { name: S },
    NumberLiteral { value: S },
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "helper::CommentDef<S>"))]
pub struct Comment<S> {
    pub content: Vec<S>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub struct CallArguments<S> {
    pub positional: Vec<InlineExpression<S>>,
    pub named: Vec<NamedArgument<S>>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub struct NamedArgument<S> {
    pub name: Identifier<S>,
    pub value: InlineExpression<S>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum InlineExpression<S> {
    StringLiteral {
        value: S,
    },
    NumberLiteral {
        value: S,
    },
    FunctionReference {
        id: Identifier<S>,
        arguments: Option<CallArguments<S>>,
    },
    MessageReference {
        id: Identifier<S>,
        attribute: Option<Identifier<S>>,
    },
    TermReference {
        id: Identifier<S>,
        attribute: Option<Identifier<S>>,
        arguments: Option<CallArguments<S>>,
    },
    VariableReference {
        id: Identifier<S>,
    },
    Placeable {
        expression: Box<Expression<S>>,
    },
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Expression<S> {
    SelectExpression {
        selector: InlineExpression<S>,
        variants: Vec<Variant<S>>,
    },
    InlineExpression(InlineExpression<S>),
}
