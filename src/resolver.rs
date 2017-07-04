use super::types::FluentType;
use super::syntax::ast;
use super::context::MessageContext;

fn eval_expr(ctx: &MessageContext, expr: &ast::Expression) -> String {
    match expr {
        &ast::Expression::MessageReference { ref id } => {
            ctx.messages
                .get(&id.name)
                .and_then(|msg| ctx.format(msg, None))
                .unwrap_or(String::from("___"))
        }
        _ => unimplemented!(),
    }
}

fn resolve_pattern(ctx: &MessageContext, pattern: &ast::Pattern) -> String {
    let result = pattern
        .elements
        .iter()
        .map(|ref elem| match elem {
                 &&ast::PatternElement::TextElement(ref s) => s.clone(),
                 &&ast::PatternElement::Expression(ref e) => eval_expr(ctx, e),
             })
        .collect::<String>();

    return result;
}

pub fn resolve(ctx: &MessageContext, message: &ast::Entry) -> FluentType {
    match message {
        &ast::Entry::Message { ref value, .. } => {
            if let &Some(ref val) = value {
                let res = resolve_pattern(ctx, &val);
                return FluentType::FluentString(res);
            };
            unimplemented!();
        }
        _ => unimplemented!(),
    };
}
