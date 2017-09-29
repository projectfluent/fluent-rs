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
        f32::from_str(&self.value).ok().map(
            |num| FluentValue::from(num),
        )
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
                env.args.and_then(|args| args.get(&id.name.as_ref())).map(
                    |arg| arg.clone(),
                )
            }
            ast::Expression::SelectExpression {
                expression: None,
                ref variants,
            } => select_default(variants).and_then(|variant| variant.value.to_value(env)),
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
            _ => unimplemented!(),
        }
    }
}

fn select_default(variants: &Vec<ast::Variant>) -> Option<&ast::Variant> {
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
