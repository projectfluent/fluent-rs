use super::scope::Scope;
use super::{ResolveContext, WriteOrResolve, WriteOrResolveContext};

use std::borrow::Borrow;
use std::fmt;

use fluent_syntax::ast;

use crate::memoizer::MemoizerKind;
use crate::resolver::ResolverError;
use crate::resource::FluentResource;
use crate::types::FluentValue;

impl<'bundle> WriteOrResolve<'bundle> for ast::Expression<&'bundle str> {
    fn write_or_resolve<'ast, 'args, 'errors, R, M, T>(
        &'ast self,
        scope: &mut Scope<'bundle, 'ast, 'args, 'errors, R, M>,
        context: &mut T,
    ) -> T::Result
    where
        R: Borrow<FluentResource>,
        M: MemoizerKind,
        T: WriteOrResolveContext<'bundle>,
    {
        match self {
            Self::Inline(exp) => exp.write_or_resolve(scope, context),
            Self::Select { selector, variants } => {
                let selector = selector.write_or_resolve(scope, &mut ResolveContext);
                match selector {
                    FluentValue::String(_) | FluentValue::Number(_) => {
                        for variant in variants {
                            let key = match variant.key {
                                ast::VariantKey::Identifier { name } => name.into(),
                                ast::VariantKey::NumberLiteral { value } => {
                                    FluentValue::try_number(value)
                                }
                            };
                            if key.matches(&selector, scope) {
                                return context.resolve_pattern(scope, &variant.value);
                            }
                        }
                    }
                    _ => {}
                }

                for variant in variants {
                    if variant.default {
                        return context.resolve_pattern(scope, &variant.value);
                    }
                }
                scope.add_error(ResolverError::MissingDefault);
                context.error(self, true)
            }
        }
    }

    fn write_error<W>(&self, w: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match self {
            Self::Inline(exp) => exp.write_error(w),
            Self::Select { selector, .. } => selector.write_error(w),
        }
    }
}
