use fluent_syntax::ast;
use fluent_syntax::parser::parse;
use fluent_syntax::parser::ParserError;

rental! {
    mod rentals {
        use super::*;
        #[rental(covariant, debug)]
        pub struct FluentResource {
            string: String,
            ast: ast::Resource<'string>,
        }
    }
}

#[derive(Debug)]
pub struct FluentResource(rentals::FluentResource);

impl FluentResource {
    pub fn try_new(source: String) -> Result<Self, (Self, Vec<ParserError>)> {
        let mut errors = None;
        let res = rentals::FluentResource::new(source, |s| match parse(s) {
            Ok(ast) => ast,
            Err((ast, err)) => {
                errors = Some(err);
                ast
            }
        });

        if let Some(errors) = errors {
            return Err((Self(res), errors));
        } else {
            return Ok(Self(res));
        }
    }

    pub fn ast<'a>(&'a self) -> &ast::Resource<'a> {
        self.0.all().ast
    }

    pub fn string<'a>(&'a self) -> &'a str {
        self.0.all().string
    }
}
