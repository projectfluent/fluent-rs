use std::collections::HashMap;

use super::types::FluentType;
use super::syntax::ast;
use super::context::MessageContext;
use super::context::FluentArgument;

struct Env<'a> {
    ctx: &'a MessageContext,
    args: Option<&'a HashMap<&'a str, FluentArgument>>,
}

fn eval_expr(env: &Env, expr: &ast::Expression) -> String {
    match *expr {
        ast::Expression::MessageReference { ref id } => {
            env.ctx
                .get_message(&id.name)
                .and_then(|msg| env.ctx.format(msg, env.args))
                .unwrap_or(String::from("___"))
        }
        ast::Expression::ExternalArgument { ref id } => {
            if let Some(args) = env.args {
                if let Some(arg) = args.get(&id.name.as_ref()) {
                    match *arg {
                        FluentArgument::String(ref s) => {
                            return s.clone();
                        }
                        FluentArgument::Number(ref n) => {
                            return format!("{}", n);
                        }
                    }
                }
            }
            String::from("___")
        }
        ast::Expression::SelectExpression {
            ref expression,
            ref variants,
        } => {
            let exp = match *expression {
                Some(ref e) => Some(eval_expr(env, &*e)),
                None => None,
            };

            for variant in variants {
                if let Some(ref e) = exp {
                    match variant.key {
                        ast::VarKey::Symbol(ref s) => {
                            if e == &s.name {
                                return resolve_pattern(env, &variant.value);
                            }
                        }
                        ast::VarKey::Number(ref i) => {
                            if e == &i.value {
                                return resolve_pattern(env, &variant.value);
                            }
                        }
                    }
                }
            }
            for variant in variants {
                if variant.default {
                    return resolve_pattern(env, &variant.value);
                }
            }
            String::from("___")
        }
        _ => unimplemented!(),
    }
}

fn resolve_pattern(env: &Env, pattern: &ast::Pattern) -> String {
    pattern
        .elements
        .iter()
        .map(|elem| match *elem {
            ast::PatternElement::TextElement(ref s) => s.clone(),
            ast::PatternElement::Placeable(ref p) => eval_expr(env, &p.expression),
        })
        .collect::<String>()
}

pub fn resolve(
    ctx: &MessageContext,
    args: Option<&HashMap<&str, FluentArgument>>,
    message: &ast::Entry,
) -> FluentType {
    let env = Env {
        ctx: ctx,
        args: args,
    };
    match *message {
        ast::Entry::Message { ref value, .. } => {
            if let Some(ref val) = *value {
                let res = resolve_pattern(&env, val);
                return FluentType::FluentString(res);
            };
            unimplemented!();
        }
        _ => unimplemented!(),
    };
}
