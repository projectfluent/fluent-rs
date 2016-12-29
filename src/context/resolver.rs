
use super::MessageContext;
use super::super::syntax::runtime::ast;

#[derive(Debug)]
pub enum ResolverError {
    Generic,
}

pub fn resolve(ctx: &MessageContext, message: &ast::Message) -> Result<String, ResolverError> {
    return Ok(format!("{:?}", message));
}
