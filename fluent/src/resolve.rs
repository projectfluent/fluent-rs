//! The `ResolveValue` trait resolves Fluent AST nodes to [`FluentValues`].
//!
//! This is an internal API used by [`MessageContext`] to evaluate Messages, Attributes and other
//! AST nodes to [`FluentValues`] which can be then formatted to strings.
//!
//! [`FluentValues`]: ../types/enum.FluentValue.html
//! [`MessageContext`]: ../context/struct.MessageContext.html

use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::context::MessageContext;
use super::entry::GetEntry;
use super::types::FluentValue;
use fluent_syntax::ast;

#[derive(Debug)]
pub enum FluentError {
    None,
    Cyclic,
}

/// State for a single `ResolveValue::to_value` call.
pub struct Env<'env> {
    /// The current `MessageContext` instance.
    pub ctx: &'env MessageContext<'env>,
    /// The current arguments passed by the developer.
    pub args: Option<&'env HashMap<&'env str, FluentValue>>,
    /// Tracks hashes to prevent infinite recursion.
    pub travelled: RefCell<Vec<u64>>,
}

impl<'env> Env<'env> {
    fn track<F>(&self, identifier: &str, action: F) -> Result<FluentValue, FluentError>
    where
        F: FnMut() -> Result<FluentValue, FluentError>,
    {
        let mut hasher = DefaultHasher::new();
        identifier.hash(&mut hasher);
        let hash = hasher.finish();

        if self.travelled.borrow().contains(&hash) {
            Err(FluentError::Cyclic)
        } else {
            self.travelled.borrow_mut().push(hash);
            self.scope(action)
        }
    }

    fn scope<T, F: FnMut() -> T>(&self, mut action: F) -> T {
        let level = self.travelled.borrow().len();
        let output = action();
        self.travelled.borrow_mut().truncate(level);
        output
    }
}

/// Converts an AST node to a `FluentValue`.
pub trait ResolveValue {
    fn to_value(&self, env: &Env) -> Result<FluentValue, FluentError>;
}

impl ResolveValue for ast::Message {
    fn to_value(&self, env: &Env) -> Result<FluentValue, FluentError> {
        env.track(&self.id.name, || {
            self.value.as_ref().ok_or(FluentError::None)?.to_value(env)
        })
    }
}

impl ResolveValue for ast::Term {
    fn to_value(&self, env: &Env) -> Result<FluentValue, FluentError> {
        env.track(&self.id.name, || self.value.to_value(env))
    }
}

impl ResolveValue for ast::Attribute {
    fn to_value(&self, env: &Env) -> Result<FluentValue, FluentError> {
        env.track(&self.id.name, || self.value.to_value(env))
    }
}

impl ResolveValue for ast::Pattern {
    fn to_value(&self, env: &Env) -> Result<FluentValue, FluentError> {
        let mut string = String::with_capacity(128);
        for elem in &self.elements {
            let result: Result<String, ()> = env.scope(|| match elem.to_value(env) {
                Err(FluentError::Cyclic) => Err(()),
                Err(_) => Ok("___".into()),
                Ok(elem) => Ok(elem.format(env.ctx)),
            });

            match result {
                Err(()) => return Ok("___".into()),
                Ok(value) => {
                    string.push_str(&value);
                }
            }
        }
        string.shrink_to_fit();
        Ok(FluentValue::from(string))
    }
}

impl ResolveValue for ast::PatternElement {
    fn to_value(&self, env: &Env) -> Result<FluentValue, FluentError> {
        match self {
            ast::PatternElement::TextElement(s) => Ok(FluentValue::from(s.clone())),
            ast::PatternElement::Placeable(p) => p.to_value(env),
        }
    }
}

impl ResolveValue for ast::Number {
    fn to_value(&self, _env: &Env) -> Result<FluentValue, FluentError> {
        FluentValue::as_number(&self.value).map_err(|_| FluentError::None)
    }
}

impl ResolveValue for ast::VariantName {
    fn to_value(&self, _env: &Env) -> Result<FluentValue, FluentError> {
        Ok(FluentValue::from(self.name.clone()))
    }
}

impl ResolveValue for ast::Expression {
    fn to_value(&self, env: &Env) -> Result<FluentValue, FluentError> {
        match self {
            ast::Expression::StringExpression { value } => Ok(FluentValue::from(value.clone())),
            ast::Expression::NumberExpression { value } => value.to_value(env),
            ast::Expression::MessageReference { ref id } if id.name.starts_with('-') => env
                .ctx
                .entries
                .get_term(&id.name)
                .ok_or(FluentError::None)?
                .to_value(env),
            ast::Expression::MessageReference { ref id } => env
                .ctx
                .entries
                .get_message(&id.name)
                .ok_or(FluentError::None)?
                .to_value(env),
            ast::Expression::ExternalArgument { ref id } => env
                .args
                .and_then(|args| args.get(&id.name.as_ref()))
                .cloned()
                .ok_or(FluentError::None),
            ast::Expression::SelectExpression {
                expression: None,
                variants,
            } => select_default(variants)
                .ok_or(FluentError::None)?
                .value
                .to_value(env),
            ast::Expression::SelectExpression {
                expression,
                variants,
            } => {
                let selector = expression.as_ref().ok_or(FluentError::None)?.to_value(env);

                if let Ok(ref selector) = selector {
                    for variant in variants {
                        match variant.key {
                            ast::VarKey::VariantName(ref symbol) => {
                                let key = FluentValue::from(symbol.name.clone());
                                if key.matches(env.ctx, selector) {
                                    return variant.value.to_value(env);
                                }
                            }
                            ast::VarKey::Number(ref number) => {
                                if let Ok(key) = number.to_value(env) {
                                    if key.matches(env.ctx, selector) {
                                        return variant.value.to_value(env);
                                    }
                                }
                            }
                        }
                    }
                }

                select_default(variants)
                    .ok_or(FluentError::None)?
                    .value
                    .to_value(env)
            }
            ast::Expression::AttributeExpression { id, name } => {
                let attributes = if id.name.starts_with('-') {
                    env.ctx
                        .entries
                        .get_term(&id.name)
                        .ok_or(FluentError::None)?
                        .attributes
                        .as_ref()
                } else {
                    env.ctx
                        .entries
                        .get_message(&id.name)
                        .ok_or(FluentError::None)?
                        .attributes
                        .as_ref()
                };
                if let Some(attributes) = attributes {
                    for attribute in attributes {
                        if attribute.id.name == name.name {
                            return attribute.to_value(env);
                        }
                    }
                }
                Err(FluentError::None)
            }
            ast::Expression::VariantExpression { id, key } if id.name.starts_with('-') => {
                let term = env
                    .ctx
                    .entries
                    .get_term(&id.name)
                    .ok_or(FluentError::None)?;
                let variants = match term.value.elements.as_slice() {
                    [ast::PatternElement::Placeable(ast::Expression::SelectExpression {
                        expression: None,
                        ref variants,
                    })] => variants,
                    _ => return term.value.to_value(env),
                };

                for variant in variants {
                    if variant.key == *key {
                        return variant.value.to_value(env);
                    }
                }

                select_default(variants)
                    .ok_or(FluentError::None)?
                    .value
                    .to_value(env)
            }
            ast::Expression::CallExpression {
                ref callee,
                ref args,
            } => {
                let resolved_unnamed_args = &mut Vec::new();
                let resolved_named_args = &mut HashMap::new();

                for arg in args {
                    env.scope(|| match arg {
                        ast::Argument::Expression(ref expression) => {
                            resolved_unnamed_args.push(expression.to_value(env).ok());
                        }
                        ast::Argument::NamedArgument { ref name, ref val } => {
                            let mut fluent_val: FluentValue;

                            match val {
                                ast::ArgValue::Number(ref num) => {
                                    fluent_val = num.to_value(env).unwrap();
                                }
                                ast::ArgValue::String(ref string) => {
                                    fluent_val = FluentValue::from(string.as_str());
                                }
                            };

                            resolved_named_args.insert(name.name.clone(), fluent_val);
                        }
                    });
                }

                env.ctx
                    .entries
                    .get_function(&callee.name)
                    .ok_or(FluentError::None)
                    .and_then(|func| {
                        func(resolved_unnamed_args.as_slice(), &resolved_named_args)
                            .ok_or(FluentError::None)
                    })
            }
            _ => unimplemented!(),
        }
    }
}

fn select_default(variants: &[ast::Variant]) -> Option<&ast::Variant> {
    for variant in variants {
        if variant.default {
            return Some(variant);
        }
    }

    None
}
