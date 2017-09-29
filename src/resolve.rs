use std::collections::HashMap;
use std::str::FromStr;

use super::types::FluentValue;
use super::syntax::ast;
use super::context::MessageContext;

struct Env<'a> {
    ctx: &'a MessageContext,
    args: Option<&'a HashMap<&'a str, FluentValue>>,
}

trait ResolveValue {
    fn to_value(&self, env: &Env) -> Option<FluentValue>;
}

impl ResolveValue for ast::Pattern {
    fn to_value(&self, env: &Env) -> Option<FluentValue> {
        let string = self.elements
            .iter()
            .map(|elem| {
                elem.to_value(env).map_or(
                    String::from("___"),
                    |elem| elem.format(),
                )
            })
            .collect::<String>();
        Some(FluentValue::from(string))
    }
}

impl ResolveValue for ast::PatternElement {
    fn to_value(&self, env: &Env) -> Option<FluentValue> {
        match *self {
            ast::PatternElement::TextElement(ref s) => Some(FluentValue::from(s.clone())),
            ast::PatternElement::Placeable(ref p) => p.to_value(env),
        }
    }
}

impl ResolveValue for ast::Placeable {
    fn to_value(&self, env: &Env) -> Option<FluentValue> {
        self.expression.to_value(env)
    }
}

impl ResolveValue for ast::Number {
    fn to_value(&self, _env: &Env) -> Option<FluentValue> {
        f32::from_str(&self.value).ok().map(FluentValue::from)
    }
}

impl ResolveValue for ast::Symbol {
    fn to_value(&self, _env: &Env) -> Option<FluentValue> {
        Some(FluentValue::from(self.name.clone()))
    }
}

impl ResolveValue for ast::Expression {
    fn to_value(&self, env: &Env) -> Option<FluentValue> {
        match *self {
            ast::Expression::StringExpression { ref value } => {
                Some(FluentValue::from(value.clone()))
            }
            ast::Expression::NumberExpression { ref value } => value.to_value(env),
            ast::Expression::MessageReference { ref id } => {
                env.ctx
                    .get_message(&id.name)
                    .and_then(|message| message.value.as_ref())
                    .and_then(|pattern| pattern.to_value(env))
            }
            ast::Expression::ExternalArgument { ref id } => {
                env.args
                    .and_then(|args| args.get(&id.name.as_ref()))
                    .cloned()
            }
            ast::Expression::SelectExpression {
                expression: None,
                ref variants,
            } => select_default(variants).and_then(|variant| variant.value.to_value(env)),
            ast::Expression::SelectExpression {
                expression: Some(box ast::Expression::MessageReference { ref id }),
                ref variants,
            } => {
                let tags = env.ctx.get_message(&id.name).and_then(
                    |message| message.tags.as_ref(),
                );

                if let Some(tags) = tags {
                    for variant in variants {
                        if let ast::VarKey::Symbol(ref symbol) = variant.key {
                            for tag in tags.iter() {
                                if symbol.name == tag.name.name {
                                    return variant.value.to_value(env);
                                }
                            }
                        }
                    }
                }

                select_default(variants).and_then(|variant| variant.value.to_value(env))
            }
            ast::Expression::SelectExpression {
                ref expression,
                ref variants,
            } => {
                let selector = expression.as_ref().and_then(|expr| expr.to_value(env));

                if let Some(selector) = selector {
                    for variant in variants {
                        match variant.key {
                            ast::VarKey::Symbol(ref symbol) => {
                                if selector == FluentValue::from(symbol.name.clone()) {
                                    return variant.value.to_value(env);
                                }
                            }
                            ast::VarKey::Number(ref number) => {
                                if let Some(number) = number.to_value(env) {
                                    if selector == number {
                                        return variant.value.to_value(env);
                                    }
                                }
                            }
                        }
                    }
                }

                select_default(variants).and_then(|variant| variant.value.to_value(env))
            }
            ast::Expression::AttributeExpression { ref id, ref name } => {
                let attributes = env.ctx.get_message(&id.name).as_ref().and_then(|message| {
                    message.attributes.as_ref()
                });
                if let Some(attributes) = attributes {
                    for attribute in attributes {
                        if attribute.id.name == name.name {
                            return attribute.value.to_value(env);
                        }
                    }
                }
                None
            }
            ast::Expression::VariantExpression { ref id, ref key } => {
                let message = env.ctx.get_message(&id.name);
                let variants = message
                    .as_ref()
                    .and_then(|message| message.value.as_ref())
                    .and_then(|pattern| {
                        if pattern.elements.len() > 1 {
                            return None;
                        }

                        match pattern.elements.first() {
                            Some(&ast::PatternElement::Placeable(ast::Placeable {
                           expression: ast::Expression::SelectExpression {
                               expression: None, ref variants
                           }
                       })) => Some(variants),
                            _ => None,
                        }
                    });

                if let Some(variants) = variants {
                    for variant in variants {
                        if variant.key == *key {
                            return variant.value.to_value(env);
                        }
                    }

                    return select_default(variants).and_then(|variant| variant.value.to_value(env));
                }

                message
                    .as_ref()
                    .and_then(|message| message.value.as_ref())
                    .and_then(|pattern| pattern.to_value(env))

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

pub fn resolve(
    ctx: &MessageContext,
    args: Option<&HashMap<&str, FluentValue>>,
    message: &ast::Message,
) -> Option<FluentValue> {
    let env = Env { ctx, args };
    message.value.as_ref().and_then(
        |pattern| pattern.to_value(&env),
    )
}
