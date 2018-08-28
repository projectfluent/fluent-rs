use fluent_syntax::ast;
use fluent_syntax::parser::errors::ParserError;
use fluent_syntax::parser::parse;

pub struct FluentResource {
    pub ast: ast::Resource,
}

impl FluentResource {
    pub fn from_string(source: &str) -> Result<Self, (Self, Vec<ParserError>)> {
        match parse(source) {
            Ok(ast) => Ok(FluentResource { ast }),
            Err((ast, errors)) => Err((FluentResource { ast }, errors)),
        }
    }
}
