use super::scope::Scope;
use super::WriteValue;

use std::borrow::Borrow;
use std::fmt;

use fluent_syntax::ast;

use crate::memoizer::MemoizerKind;
use crate::resolver::ResolveValue;
use crate::resource::FluentResource;
use crate::types::FluentValue;

impl<'bundle> WriteValue<'bundle> for ast::Expression<&'bundle str> {
    fn write<'ast, 'args, 'errors, W, R, M>(
        &'ast self,
        w: &mut W,
        scope: &mut Scope<'bundle, 'ast, 'args, 'errors, R, M>,
    ) -> fmt::Result
    where
        W: fmt::Write,
        R: Borrow<FluentResource>,
        M: MemoizerKind,
    {
        match self {
            Self::Inline(exp) => exp.write(w, scope),
            Self::Select { selector, variants } => {
                let selector = selector.resolve(scope);
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
                                return variant.value.write(w, scope);
                            }
                        }
                    }
                    _ => {}
                }

                variants
                    .iter()
                    .find(|variant| variant.default)
                    .expect("select expressions have a default variant")
                    .value
                    .write(w, scope)
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

impl<'bundle> ResolveValue<'bundle> for ast::Expression<&'bundle str> {
    fn resolve<'ast, 'args, 'errors, R, M>(
        &'ast self,
        scope: &mut Scope<'bundle, 'ast, 'args, 'errors, R, M>,
    ) -> FluentValue<'bundle>
    where
        R: Borrow<FluentResource>,
        M: MemoizerKind,
    {
        match self {
            Self::Inline(exp) => exp.resolve(scope),
            Self::Select { selector, variants } => {
                let selector = selector.resolve(scope);
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
                                return variant.value.resolve(scope);
                            }
                        }
                    }
                    _ => {}
                }

                variants
                    .iter()
                    .find(|variant| variant.default)
                    .expect("select expressions have a default variant")
                    .value
                    .resolve(scope)
            }
        }
    }
}
