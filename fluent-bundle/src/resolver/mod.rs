//! The `ResolveValue` trait resolves Fluent AST nodes to [`FluentValues`].
//!
//! This is an internal API used by [`FluentBundle`] to evaluate Messages, Attributes and other
//! AST nodes to [`FluentValues`] which can be then formatted to strings.
//!
//! [`FluentValues`]: ../types/enum.FluentValue.html
//! [`FluentBundle`]: ../bundle/struct.FluentBundle.html

mod errors;
mod expression;
mod inline_expression;
mod pattern;
mod scope;

pub use errors::ResolverError;
pub use scope::Scope;

use std::borrow::Borrow;
// use std::fmt::Write;
use std::fmt;

// use fluent_syntax::ast;
// use fluent_syntax::unicode::unescape_unicode;

// use crate::bundle::FluentArgs;
// use crate::entry::GetEntry;
use crate::memoizer::MemoizerKind;
use crate::resource::FluentResource;
// use crate::types::DisplayableNode;
use crate::types::FluentValue;

// const MAX_PLACEABLES: u8 = 100;

// Converts an AST node to a `FluentValue`.
pub(crate) trait ResolveValue {
    fn resolve<'source, R, M: MemoizerKind>(
        &'source self,
        scope: &mut Scope<'source, R, M>,
    ) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>,
    {
        unimplemented!();
    }

    fn resolve_error(&self) -> String {
        unimplemented!();
    }
}

pub(crate) trait WriteValue {
    fn write<'source, W, R, M: MemoizerKind>(
        &'source self,
        w: &mut W,
        scope: &mut Scope<'source, R, M>,
    ) -> fmt::Result
    where
        W: fmt::Write,
        R: Borrow<FluentResource>,
    {
        unimplemented!();
    }

    fn write_error<W>(&self, _w: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        unimplemented!();
    }
}
