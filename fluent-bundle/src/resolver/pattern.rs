use super::scope::Scope;
use super::{ResolverError, WriteValue};

use std::borrow::Borrow;
use std::fmt;

use fluent_syntax::ast;

use crate::memoizer::MemoizerKind;
use crate::resolver::ResolveValue;
use crate::resource::FluentResource;
use crate::types::FluentValue;

const MAX_PLACEABLES: u8 = 100;

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
        let len = self.elements.len();

        for elem in &self.elements {
            if scope.dirty {
                w.write_str("???")?;
                return Ok(());
            }

            match elem {
                ast::PatternElement::TextElement(s) => {
                    if let Some(ref transform) = scope.bundle.transform {
                        w.write_str(&transform(s))?;
                    } else {
                        w.write_str(s)?;
                    }
                }
                ast::PatternElement::Placeable(ref p) => {
                    scope.placeables += 1;
                    if scope.placeables > MAX_PLACEABLES {
                        scope.dirty = true;
                        scope.add_error(ResolverError::TooManyPlaceables);
                        return w.write_str("???");
                    }

                    let needs_isolation = scope.bundle.use_isolating
                        && len > 1
                        && match p {
                            ast::Expression::InlineExpression(
                                ast::InlineExpression::MessageReference { .. },
                            )
                            | ast::Expression::InlineExpression(
                                ast::InlineExpression::TermReference { .. },
                            )
                            | ast::Expression::InlineExpression(
                                ast::InlineExpression::StringLiteral { .. },
                            ) => false,
                            _ => true,
                        };
                    if needs_isolation {
                        w.write_char('\u{2068}')?;
                    }
                    p.write(w, scope)?;
                    if needs_isolation {
                        w.write_char('\u{2069}')?;
                    }
                }
            }
        }
        Ok(())
    }

    fn write_error<W>(&self, _w: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        Ok(())
    }
}

impl<'p> ResolveValue for ast::Pattern<'p> {
    fn resolve<'source, R, M: MemoizerKind>(
        &'source self,
        scope: &mut Scope<'source, R, M>,
    ) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>,
    {
        let len = self.elements.len();

        if len == 1 {
            match self.elements[0] {
                ast::PatternElement::TextElement(s) => return s.into(),
                _ => {}
            }
        }

        let mut result = String::new();
        self.write(&mut result, scope).unwrap();
        result.into()
    }

    fn resolve_error(&self) -> String {
        unreachable!()
    }
}
