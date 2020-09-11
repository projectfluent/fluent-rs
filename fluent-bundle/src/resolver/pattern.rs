use super::scope::Scope;
use super::WriteValue;

use std::borrow::Borrow;
use std::fmt;

use fluent_syntax::ast;

use crate::memoizer::MemoizerKind;
use crate::resource::FluentResource;

impl<'p> WriteValue for ast::Pattern<'p> {
    fn write<W, R, M: MemoizerKind>(&self, w: &mut W, scope: &mut Scope<R, M>) -> fmt::Result
    where
        W: fmt::Write,
        R: Borrow<FluentResource>,
    {
        if scope.dirty {
            w.write_str("???")?;
            return Ok(());
        }
        if self.elements.len() == 1 {
            match self.elements[0] {
                ast::PatternElement::TextElement(s) => {
                    w.write_str(s)?;
                }
                ast::PatternElement::Placeable(ref p) => {
                    unimplemented!();
                }
            };
        }
        Ok(())
    }
}
