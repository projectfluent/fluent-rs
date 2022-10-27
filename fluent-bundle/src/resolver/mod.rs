pub mod errors;
mod expression;
mod inline_expression;
mod pattern;
mod scope;

pub use errors::ResolverError;
pub use scope::Scope;

use std::borrow::Borrow;
use std::fmt;

use crate::memoizer::MemoizerKind;
use crate::resource::FluentResource;
use crate::types::FluentValue;

// Converts an AST node to a `FluentValue`.
pub(crate) trait ResolveValue<'bundle> {
    fn resolve<'ast, 'args, 'errors, R, M>(
        &'ast self,
        scope: &mut Scope<'bundle, 'ast, 'args, 'errors, R, M>,
    ) -> FluentValue<'bundle>
    where
        R: Borrow<FluentResource>,
        M: MemoizerKind;
}

pub(crate) trait WriteValue<'bundle> {
    fn write<'ast, 'args, 'errors, W, R, M>(
        &'ast self,
        w: &mut W,
        scope: &mut Scope<'bundle, 'ast, 'args, 'errors, R, M>,
    ) -> fmt::Result
    where
        W: fmt::Write,
        R: Borrow<FluentResource>,
        M: MemoizerKind;

    fn write_error<W>(&self, _w: &mut W) -> fmt::Result
    where
        W: fmt::Write;
}
