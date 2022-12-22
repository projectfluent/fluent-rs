use super::scope::Scope;
use super::{ResolverError, WriteOrResolve, WriteOrResolveContext};

use crate::entry::GetEntry;
use crate::memoizer::MemoizerKind;
use crate::resource::FluentResource;
use crate::types::FluentValue;
use fluent_syntax::ast;
use std::borrow::{Borrow, Cow};
use std::fmt;

impl<'bundle> WriteOrResolve<'bundle> for ast::InlineExpression<&'bundle str> {
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
            Self::StringLiteral { value } => context.unescape(value),
            Self::MessageReference { id, attribute } => {
                if let Some(msg) = scope.bundle.get_entry_message(id.name) {
                    if let Some(attr) = attribute {
                        msg.attributes
                            .iter()
                            .find_map(|a| {
                                if a.id.name == attr.name {
                                    Some(scope.track(context, &a.value, self))
                                } else {
                                    None
                                }
                            })
                            .unwrap_or_else(|| {
                                scope.add_error(self.into());
                                context.error(self, true)
                            })
                    } else {
                        msg.value
                            .as_ref()
                            .map(|value| scope.track(context, value, self))
                            .unwrap_or_else(|| {
                                scope.add_error(ResolverError::NoValue(id.name.to_string()));
                                context.error(self, true)
                            })
                    }
                } else {
                    scope.add_error(self.into());
                    context.error(self, true)
                }
            }
            Self::NumberLiteral { value } => {
                context.value(scope, Cow::Owned(FluentValue::try_number(value)))
            }
            Self::TermReference {
                id,
                attribute,
                arguments,
            } => {
                let (_, resolved_named_args) = scope.get_arguments(arguments.as_ref());

                scope.local_args = Some(resolved_named_args);
                let result = scope
                    .bundle
                    .get_entry_term(id.name)
                    .and_then(|term| {
                        if let Some(attr) = attribute {
                            term.attributes.iter().find_map(|a| {
                                if a.id.name == attr.name {
                                    Some(scope.track(context, &a.value, self))
                                } else {
                                    None
                                }
                            })
                        } else {
                            Some(scope.track(context, &term.value, self))
                        }
                    })
                    .unwrap_or_else(|| {
                        scope.add_error(self.into());
                        context.error(self, true)
                    });
                scope.local_args = None;
                result
            }
            Self::FunctionReference { id, arguments } => {
                let (resolved_positional_args, resolved_named_args) =
                    scope.get_arguments(Some(arguments));

                let func = scope.bundle.get_entry_function(id.name);

                if let Some(func) = func {
                    let result = func(resolved_positional_args.as_slice(), &resolved_named_args);
                    if matches!(result, FluentValue::Error) {
                        context.error(self, false)
                    } else {
                        context.value(scope, Cow::Owned(result))
                    }
                } else {
                    scope.add_error(self.into());
                    context.error(self, true)
                }
            }
            Self::VariableReference { id } => {
                if let Some(local_args) = &scope.local_args {
                    if let Some(arg) = local_args.get(id.name) {
                        return context.value(scope, Cow::Borrowed(arg));
                    }
                } else if let Some(arg) = scope.args.and_then(|args| args.get(id.name)) {
                    return context.value(scope, Cow::Owned(arg.into_owned()));
                }

                if scope.local_args.is_none() {
                    scope.add_error(self.into());
                }
                context.error(self, true)
            }
            Self::Placeable { expression } => expression.write_or_resolve(scope, context),
        }
    }

    fn write_error<W>(&self, w: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match self {
            Self::MessageReference {
                id,
                attribute: Some(attribute),
            } => write!(w, "{}.{}", id.name, attribute.name),
            Self::MessageReference {
                id,
                attribute: None,
            } => w.write_str(id.name),
            Self::TermReference {
                id,
                attribute: Some(attribute),
                ..
            } => write!(w, "-{}.{}", id.name, attribute.name),
            Self::TermReference {
                id,
                attribute: None,
                ..
            } => write!(w, "-{}", id.name),
            Self::FunctionReference { id, .. } => write!(w, "{}()", id.name),
            Self::VariableReference { id } => write!(w, "${}", id.name),
            _ => unreachable!(),
        }
    }
}
