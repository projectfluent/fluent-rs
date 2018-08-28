use super::resolve::ResolverError;
use fluent_syntax::parser::errors::ParserError;

#[derive(Debug, Fail, PartialEq)]
pub enum FluentError {
    #[fail(
        display = "attempted to override an existing {}: {}",
        kind,
        id
    )]
    Overriding { kind: &'static str, id: String },
    #[fail(display = "Parser error")]
    ParserError(ParserError),
    #[fail(display = "Resolver error")]
    ResolverError(ResolverError),
}

impl From<ParserError> for FluentError {
    fn from(error: ParserError) -> Self {
        FluentError::ParserError(error)
    }
}

impl From<ResolverError> for FluentError {
    fn from(error: ResolverError) -> Self {
        FluentError::ResolverError(error)
    }
}
