use super::scope::Scope;
use super::{ResolveValue, WriteValue};

use std::borrow::Borrow;
use std::fmt;

use fluent_syntax::ast;

use crate::memoizer::MemoizerKind;
use crate::resource::FluentResource;
use crate::types::FluentValue;

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
                    let needs_isolation = scope.bundle.use_isolating
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
}

impl<'p> ResolveValue for ast::Pattern<'p> {
    fn resolve<'source, R, M: MemoizerKind>(
        &'source self,
        scope: &mut Scope<'source, R, M>,
    ) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>,
    {
        let mut string = String::new();
        self.write(&mut string, scope)
            .expect("Failed to write to string");
        string.into()
    }
}
