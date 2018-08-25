use fluent_syntax::ast;
use fluent_syntax::parser::parse;

pub struct FluentResource {
    pub ast: ast::Resource,
}

impl FluentResource {
    pub fn from_string(source: &str) -> Self {
        let ast = parse(source).unwrap_or_else(|x| x.0);
        FluentResource { ast }
    }
}
