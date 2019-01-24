//! The `ResolveValue` trait resolves Fluent AST nodes to [`FluentValues`].
//!
//! This is an internal API used by [`FluentBundle`] to evaluate Messages, Attributes and other
//! AST nodes to [`FluentValues`] which can be then formatted to strings.
//!
//! [`FluentValues`]: ../types/enum.FluentValue.html
//! [`FluentBundle`]: ../bundle/struct.FluentBundle.html

use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::bundle::FluentBundle;
use super::entry::GetEntry;
use super::types::FluentValue;
use fluent_syntax::ast;

#[derive(Debug, PartialEq)]
pub enum ResolverError {
    None,
    Cyclic,
}

/// State for a single `ResolveValue::to_value` call.
pub struct Env<'env> {
    /// The current `FluentBundle` instance.
    pub bundle: &'env FluentBundle<'env>,
    /// The current arguments passed by the developer.
    pub args: Option<&'env HashMap<&'env str, FluentValue>>,
    /// Tracks hashes to prevent infinite recursion.
    pub travelled: RefCell<Vec<u64>>,
}

impl<'env> Env<'env> {
    fn track<F>(&self, identifier: &str, action: F) -> Result<FluentValue, ResolverError>
    where
        F: FnMut() -> Result<FluentValue, ResolverError>,
    {
        let mut hasher = DefaultHasher::new();
        identifier.hash(&mut hasher);
        let hash = hasher.finish();

        if self.travelled.borrow().contains(&hash) {
            Err(ResolverError::Cyclic)
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
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError>;
}

impl<'source> ResolveValue for ast::Message<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        env.track(&self.id.name, || {
            self.value
                .as_ref()
                .ok_or(ResolverError::None)?
                .to_value(env)
        })
    }
}

impl<'source> ResolveValue for ast::Term<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        env.track(&self.id.name, || self.value.to_value(env))
    }
}

impl<'source> ResolveValue for ast::Attribute<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        env.track(&self.id.name, || self.value.to_value(env))
    }
}

impl<'source> ResolveValue for ast::Value<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        match self {
            ast::Value::Pattern(p) => p.to_value(env),
            ast::Value::VariantList { variants } => select_default(variants)
                .ok_or(ResolverError::None)?
                .value
                .to_value(env),
        }
    }
}

impl<'source> ResolveValue for ast::Pattern<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        let mut string = String::with_capacity(128);
        for elem in &self.elements {
            let result: Result<String, ()> = env.scope(|| match elem.to_value(env) {
                Err(ResolverError::Cyclic) => Err(()),
                Err(_) => Ok("___".into()),
                Ok(elem) => Ok(elem.format(env.bundle)),
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

impl<'source> ResolveValue for ast::PatternElement<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        match self {
            ast::PatternElement::TextElement(s) => Ok(FluentValue::from(*s)),
            ast::PatternElement::Placeable(p) => p.to_value(env),
        }
    }
}

impl<'source> ResolveValue for ast::VariantKey<'source> {
    fn to_value(&self, _env: &Env) -> Result<FluentValue, ResolverError> {
        match self {
            ast::VariantKey::Identifier { name } => Ok(FluentValue::from(*name)),
            ast::VariantKey::NumberLiteral { value } => {
                FluentValue::as_number(value).map_err(|_| ResolverError::None)
            }
        }
    }
}

impl<'source> ResolveValue for ast::Expression<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        match self {
            ast::Expression::InlineExpression(exp) => exp.to_value(env),
            ast::Expression::SelectExpression { selector, variants } => {
                if let Ok(ref selector) = selector.to_value(env) {
                    for variant in variants {
                        match variant.key {
                            ast::VariantKey::Identifier { name } => {
                                let key = FluentValue::from(name);
                                if key.matches(env.bundle, selector) {
                                    return variant.value.to_value(env);
                                }
                            }
                            ast::VariantKey::NumberLiteral { value } => {
                                let key = FluentValue::as_number(value).unwrap();
                                if key.matches(env.bundle, selector) {
                                    return variant.value.to_value(env);
                                }
                            }
                        }
                    }
                }

                select_default(variants)
                    .ok_or(ResolverError::None)?
                    .value
                    .to_value(env)
            }
        }
    }
}

impl<'source> ResolveValue for ast::InlineExpression<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        match self {
            ast::InlineExpression::StringLiteral { raw } => {
                // XXX: We need to decode the raw into unicode here.
                Ok(FluentValue::from(*raw))
            }
            ast::InlineExpression::NumberLiteral { value } => {
                Ok(FluentValue::as_number(*value).unwrap())
            }
            ast::InlineExpression::VariableReference { id } => env
                .args
                .and_then(|args| args.get(&id.name))
                .cloned()
                .ok_or(ResolverError::None),
            ast::InlineExpression::CallExpression {
                ref callee,
                ref positional,
                ref named,
            } => {
                let mut resolved_positional_args = Vec::new();
                let mut resolved_named_args = HashMap::new();

                for expression in positional {
                    resolved_positional_args.push(expression.to_value(env).ok());
                }

                for arg in named {
                    resolved_named_args
                        .insert(arg.name.name.to_string(), arg.value.to_value(env).unwrap());
                }

                let func = match **callee {
                    ast::InlineExpression::FunctionReference { ref id } => {
                        env.bundle.entries.get_function(id.name)
                    }
                    _ => panic!(),
                };

                func.ok_or(ResolverError::None).and_then(|func| {
                    func(resolved_positional_args.as_slice(), &resolved_named_args)
                        .ok_or(ResolverError::None)
                })
            }
            ast::InlineExpression::AttributeExpression { reference, name } => {
                let attributes: &Vec<ast::Attribute> = match reference.as_ref() {
                    ast::InlineExpression::MessageReference { ref id } => env
                        .bundle
                        .entries
                        .get_message(&id.name)
                        .ok_or(ResolverError::None)?
                        .attributes
                        .as_ref(),
                    ast::InlineExpression::TermReference { ref id } => env
                        .bundle
                        .entries
                        .get_term(&id.name)
                        .ok_or(ResolverError::None)?
                        .attributes
                        .as_ref(),
                    _ => unimplemented!(),
                };
                for attribute in attributes {
                    if attribute.id.name == name.name {
                        return attribute.to_value(env);
                    }
                }
                Err(ResolverError::None)
            }
            ast::InlineExpression::VariantExpression { reference, key } => {
                if let ast::InlineExpression::TermReference { ref id } = reference.as_ref() {
                    let term = env
                        .bundle
                        .entries
                        .get_term(&id.name)
                        .ok_or(ResolverError::None)?;

                    match term.value {
                        ast::Value::VariantList { ref variants } => {
                            for variant in variants {
                                if variant.key == *key {
                                    return variant.value.to_value(env);
                                }
                            }

                            select_default(variants)
                                .ok_or(ResolverError::None)?
                                .value
                                .to_value(env)
                        }
                        ast::Value::Pattern(ref p) => p.to_value(env),
                    }
                } else {
                    unimplemented!()
                }
            }
            ast::InlineExpression::FunctionReference { .. } => panic!(),
            ast::InlineExpression::MessageReference { ref id } => env
                .bundle
                .entries
                .get_message(&id.name)
                .ok_or(ResolverError::None)?
                .to_value(env),
            ast::InlineExpression::TermReference { ref id } => env
                .bundle
                .entries
                .get_term(&id.name)
                .ok_or(ResolverError::None)?
                .to_value(env),
            ast::InlineExpression::Placeable { ref expression } => {
                let exp = expression.as_ref();
                exp.to_value(env)
            }
        }
    }
}

fn select_default<'source>(
    variants: &'source [ast::Variant<'source>],
) -> Option<&ast::Variant<'source>> {
    for variant in variants {
        if variant.default {
            return Some(variant);
        }
    }

    None
}
