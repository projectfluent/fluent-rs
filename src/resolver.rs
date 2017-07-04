use super::types::FluentType;
use super::syntax::ast;

fn resolve_pattern(pattern: &ast::Pattern) -> String {
    let result = pattern
        .elements
        .iter()
        .map(|ref elem| match elem {
                 &&ast::PatternElement::TextElement(ref s) => s.clone(),
                 &&ast::PatternElement::Expression(ref e) => String::from("{}"),
             })
        .collect::<String>();

    return result;
}

pub fn resolve(message: &ast::Entry) -> FluentType {
    match message {
        &ast::Entry::Message { ref value, .. } => {
            if let &Some(ref val) = value {
                let res = resolve_pattern(&val);
                return FluentType::FluentString(res);
            };
            unimplemented!();
        }
        _ => unimplemented!(),
    };
}
