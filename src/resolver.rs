use std::collections::HashMap;

use super::types::FluentType;
use super::syntax::ast;
use super::context::MessageContext;
use super::context::FluentArgument;

struct Env<'a> {
    ctx: &'a MessageContext,
    args: Option<&'a HashMap<String, FluentArgument>>,
}

fn eval_expr(env: &Env, expr: &ast::Expression) -> String {
    match expr {
        &ast::Expression::MessageReference { ref id } => {
            env.ctx
                .messages
                .get(&id.name)
                .and_then(|msg| env.ctx.format(msg, env.args))
                .unwrap_or(String::from("___"))
        }
        &ast::Expression::ExternalArgument { ref id } => {
            if let Some(args) = env.args {
                if let Some(arg) = args.get(&id.name) {
                    match arg {
                        &FluentArgument::String(ref s) => {
                            return s.clone();
                        }
                        &FluentArgument::Number(ref n) => {
                            return format!("{}", n);
                        }
                    }
                }
            }
            return String::from("___");
        }
        _ => unimplemented!(),
    }
}

fn resolve_pattern(env: &Env, pattern: &ast::Pattern) -> String {
    let result = pattern
        .elements
        .iter()
        .map(|ref elem| match elem {
                 &&ast::PatternElement::TextElement(ref s) => s.clone(),
                 &&ast::PatternElement::Expression(ref e) => eval_expr(env, e),
             })
        .collect::<String>();

    return result;
}

pub fn resolve(ctx: &MessageContext,
               args: Option<&HashMap<String, FluentArgument>>,
               message: &ast::Entry)
               -> FluentType {
    let env = Env {
        ctx: ctx,
        args: args,
    };
    match message {
        &ast::Entry::Message { ref value, .. } => {
            if let &Some(ref val) = value {
                let res = resolve_pattern(&env, &val);
                return FluentType::FluentString(res);
            };
            unimplemented!();
        }
        _ => unimplemented!(),
    };
}
