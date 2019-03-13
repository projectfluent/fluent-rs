use super::resolve::ResolverError;
use super::types::FluentValueError;
use fluent_syntax::parser::ParserError;

#[derive(Debug, Fail, PartialEq)]
pub enum FluentError {
    #[fail(display = "attempted to override an existing {}: {}", kind, id)]
    Overriding { kind: &'static str, id: String },
    #[fail(display = "Parser error")]
    ParserError(ParserError),
    #[fail(display = "Resolver error")]
    ResolverError(ResolverError),
    #[fail(display = "FluentValue error")]
    FluentValueError(FluentValueError),
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

impl From<FluentValueError> for FluentError {
    fn from(error: FluentValueError) -> Self {
        FluentError::FluentValueError(error)
    }
}
