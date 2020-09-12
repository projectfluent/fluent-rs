use super::scope::Scope;
use super::WriteValue;

use std::borrow::Borrow;
use std::fmt;

use fluent_syntax::ast;

use crate::memoizer::MemoizerKind;
use crate::resource::FluentResource;

impl<'p> WriteValue for ast::Pattern<'p> {
    fn write<'scope, W, R, M: MemoizerKind>(
        &'scope self,
        w: &mut W,
        scope: &mut Scope<'scope, R, M>,
    ) -> fmt::Result
    where
        W: fmt::Write,
        R: Borrow<FluentResource>,
    {
        if scope.dirty {
            w.write_str("???")?;
            return Ok(());
        }

        for elem in &self.elements {
            match elem {
                ast::PatternElement::TextElement(s) => {
                    w.write_str(s)?;
                }
                ast::PatternElement::Placeable(ref p) => {
                    p.write(w, scope)?;
                }
            }
        }
        Ok(())
    }
}
