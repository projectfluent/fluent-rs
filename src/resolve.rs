//! The `ResolveValue` trait resolves Fluent AST nodes to [`FluentValues`].
//!
//! This is an internal API used by [`MessageContext`] to evaluate Messages, Attributes and other
//! AST nodes to [`FluentValues`] which can be then formatted to strings.
//!
//! [`FluentValues`]: ../types/enum.FluentValue.html
//! [`MessageContext`]: ../context/struct.MessageContext.html

use std::collections::HashMap;
use std::str::FromStr;

use super::context::MessageContext;
use super::syntax::ast;
use super::types::FluentValue;

/// State for a single `ResolveValue::to_value` call.
pub struct Env<'env> {
    /// The current `MessageContext` instance.
    pub ctx: &'env MessageContext<'env>,
    /// The current arguments passed by the developer.
    pub args: Option<&'env HashMap<&'env str, FluentValue>>,
}

/// Converts an AST node to a `FluentValue`.
pub trait ResolveValue {
    fn to_value(&self, env: &Env) -> Option<FluentValue>;
}

impl ResolveValue for ast::Message {
    fn to_value(&self, env: &Env) -> Option<FluentValue> {
        self.value.as_ref()?.to_value(env)
    }
}

impl ResolveValue for ast::Term {
    fn to_value(&self, env: &Env) -> Option<FluentValue> {
        self.value.to_value(env)
    }
}

impl ResolveValue for ast::Attribute {
    fn to_value(&self, env: &Env) -> Option<FluentValue> {
        self.value.to_value(env)
    }
}

impl ResolveValue for ast::Pattern {
    fn to_value(&self, env: &Env) -> Option<FluentValue> {
        let string = self
            .elements
            .iter()
            .map(|elem| {
                elem.to_value(env)
                    .map_or(String::from("___"), |elem| elem.format(env.ctx))
            })
            .collect::<String>();
        Some(FluentValue::from(string))
    }
}

impl ResolveValue for ast::PatternElement {
    fn to_value(&self, env: &Env) -> Option<FluentValue> {
        match self {
            ast::PatternElement::TextElement(s) => Some(FluentValue::from(s.clone())),
            ast::PatternElement::Placeable(p) => p.to_value(env),
        }
    }
}

impl ResolveValue for ast::Number {
    fn to_value(&self, _env: &Env) -> Option<FluentValue> {
        f32::from_str(&self.value).ok().map(FluentValue::from)
    }
}

impl ResolveValue for ast::VariantName {
    fn to_value(&self, _env: &Env) -> Option<FluentValue> {
        Some(FluentValue::from(self.name.clone()))
    }
}

impl ResolveValue for ast::Expression {
    fn to_value(&self, env: &Env) -> Option<FluentValue> {
        match self {
            ast::Expression::StringExpression { value } => Some(FluentValue::from(value.clone())),
            ast::Expression::NumberExpression { value } => value.to_value(env),
            ast::Expression::MessageReference { id } if id.name.starts_with('-') => {
                env.ctx.get_term(&id.name)?.to_value(env)
            }
            ast::Expression::NumberExpression { ref value } => value.to_value(env),
            ast::Expression::MessageReference { ref id } if id.name.starts_with('-') => env
                .ctx
                .get_term(&id.name)
                .and_then(|term| term.to_value(env)),
            ast::Expression::MessageReference { ref id } => env
                .ctx
                .get_message(&id.name)
                .and_then(|message| message.to_value(env)),
            ast::Expression::ExternalArgument { ref id } => env
                .args
                .and_then(|args| args.get(&id.name.as_ref()))
                .cloned(),
            ast::Expression::SelectExpression {
                expression: None,
                variants,
            } => select_default(variants)?.value.to_value(env),
            ast::Expression::SelectExpression {
                expression,
                variants,
            } => {
                let selector = expression.as_ref()?.to_value(env);

                if let Some(ref selector) = selector {
                    for variant in variants {
                        match variant.key {
                            ast::VarKey::VariantName(ref symbol) => {
                                let key = FluentValue::from(symbol.name.clone());
                                if key.matches(env.ctx, selector) {
                                    return variant.value.to_value(env);
                                }
                            }
                            ast::VarKey::Number(ref number) => {
                                if let Some(key) = number.to_value(env) {
                                    if key.matches(env.ctx, selector) {
                                        return variant.value.to_value(env);
                                    }
                                }
                            }
                        }
                    }
                }

                select_default(variants)?.value.to_value(env)
            }
            ast::Expression::AttributeExpression { id, name } => {
                let attributes = if id.name.starts_with('-') {
                    env.ctx.get_term(&id.name)?.attributes.as_ref()
                } else {
                    env.ctx.get_message(&id.name)?.attributes.as_ref()
                };
                if let Some(attributes) = attributes {
                    for attribute in attributes {
                        if attribute.id.name == name.name {
                            return attribute.to_value(env);
                        }
                    }
                }
                None
            }
            ast::Expression::VariantExpression { id, key } if id.name.starts_with('-') => {
                let term = env.ctx.get_term(&id.name)?;
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

                select_default(variants)?.value.to_value(env)
            }
            ast::Expression::CallExpression {
                ref callee,
                ref args,
            } => {
                let mut resolved_unnamed_args = Vec::new();
                let mut resolved_named_args = HashMap::new();

                for arg in args {
                    match arg {
                        &ast::Argument::Expression(ref expression) => {
                            resolved_unnamed_args.push(expression.to_value(env));
                        }
                        &ast::Argument::NamedArgument { ref name, ref val } => {
                            let mut fluent_val: FluentValue;

                            match val {
                                &ast::ArgValue::Number(ref num) => {
                                    fluent_val = num.to_value(env).unwrap();
                                }
                                &ast::ArgValue::String(ref string) => {
                                    fluent_val = FluentValue::from(string.as_str());
                                }
                            };

                            resolved_named_args.insert(name.name.clone(), fluent_val);
                        }
                    }
                }

                env.ctx
                    .get_function(&callee.name)
                    .and_then(|func| func(resolved_unnamed_args.as_slice(), &resolved_named_args))
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
