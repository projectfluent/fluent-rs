use super::scope::Scope;
use super::{ResolveValue, ResolverError, WriteValue};

use std::borrow::Borrow;
use std::fmt;

use fluent_syntax::ast;
use fluent_syntax::unicode::{unescape_unicode, unescape_unicode_to_str};

use crate::bundle::FluentArgs;
use crate::entry::GetEntry;
use crate::memoizer::MemoizerKind;
use crate::resource::FluentResource;
use crate::types::FluentValue;

impl<'p> WriteValue for ast::InlineExpression<'p> {
    fn write<'scope, W, R, M: MemoizerKind>(
        &'scope self,
        w: &mut W,
        scope: &mut Scope<'scope, R, M>,
    ) -> fmt::Result
    where
        W: fmt::Write,
        R: Borrow<FluentResource>,
    {
        match self {
            ast::InlineExpression::StringLiteral { value } => unescape_unicode(w, value),
            ast::InlineExpression::MessageReference { id, attribute } => scope
                .bundle
                .get_entry_message(&id.name)
                .and_then(|msg| {
                    if let Some(attr) = attribute {
                        msg.attributes
                            .iter()
                            .find(|a| a.id.name == attr.name)
                            .map(|attr| scope.track(w, &attr.value, self))
                    } else {
                        msg.value.as_ref().map(|value| scope.track(w, value, self))
                    }
                })
                .unwrap_or_else(|| scope.write_ref_error(w, self)),
            ast::InlineExpression::NumberLiteral { value } => {
                FluentValue::try_number(*value).write(w, scope)
            }
            ast::InlineExpression::TermReference {
                id,
                attribute,
                arguments,
            } => {
                let (_, resolved_named_args) = get_arguments(scope, arguments);

                scope.local_args = Some(resolved_named_args);
                let result = scope
                    .bundle
                    .get_entry_term(&id.name)
                    .and_then(|term| {
                        if let Some(attr) = attribute {
                            term.attributes
                                .iter()
                                .find(|a| a.id.name == attr.name)
                                .map(|attr| scope.track(w, &attr.value, self))
                        } else {
                            Some(scope.track(w, &term.value, self))
                        }
                    })
                    .unwrap_or_else(|| scope.write_ref_error(w, self));
                scope.local_args = None;
                result
            }
            ast::InlineExpression::FunctionReference { id, arguments } => {
                let (resolved_positional_args, resolved_named_args) =
                    get_arguments(scope, arguments);

                let func = scope.bundle.get_entry_function(id.name);

                if let Some(func) = func {
                    let result = func(resolved_positional_args.as_slice(), &resolved_named_args);
                    if let FluentValue::Error = result {
                        self.write_error(w)
                    } else {
                        w.write_str(&result.as_string(scope))
                    }
                } else {
                    scope.write_ref_error(w, self)
                }
            }
            ast::InlineExpression::VariableReference { id } => {
                let args = scope.local_args.as_ref().or(scope.args);

                if let Some(arg) = args.and_then(|args| args.get(id.name)) {
                    arg.write(w, scope)
                } else {
                    if scope.local_args.is_none() {
                        scope.add_error(ResolverError::Reference(self.resolve_error()));
                    }
                    w.write_char('{')?;
                    self.write_error(w)?;
                    w.write_char('}')
                }
            }
            ast::InlineExpression::Placeable { expression } => expression.write(w, scope),
        }
    }

    fn write_error<W>(&self, w: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match self {
            ast::InlineExpression::MessageReference {
                id,
                attribute: Some(attribute),
            } => write!(w, "{}.{}", id.name, attribute.name),
            ast::InlineExpression::MessageReference {
                id,
                attribute: None,
            } => w.write_str(id.name),
            ast::InlineExpression::TermReference {
                id,
                attribute: Some(attribute),
                ..
            } => write!(w, "-{}.{}", id.name, attribute.name),
            ast::InlineExpression::TermReference {
                id,
                attribute: None,
                ..
            } => write!(w, "-{}", id.name),
            ast::InlineExpression::FunctionReference { id, .. } => write!(w, "{}()", id.name),
            ast::InlineExpression::VariableReference { id } => write!(w, "${}", id.name),
            _ => unreachable!(),
        }
    }
}

impl<'p> ResolveValue for ast::InlineExpression<'p> {
    fn resolve<'source, R, M: MemoizerKind>(
        &'source self,
        scope: &mut Scope<'source, R, M>,
    ) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>,
    {
        match self {
            ast::InlineExpression::StringLiteral { value } => unescape_unicode_to_str(value).into(),
            ast::InlineExpression::MessageReference { id, attribute } => scope
                .bundle
                .get_entry_message(&id.name)
                .and_then(|msg| {
                    if let Some(attr) = attribute {
                        msg.attributes
                            .iter()
                            .find(|a| a.id.name == attr.name)
                            .map(|attr| {
                                let mut result = String::new();
                                scope.track(&mut result, &attr.value, self).unwrap();
                                result.into()
                            })
                    } else {
                        msg.value.as_ref().map(|value| {
                            let mut result = String::new();
                            scope.track(&mut result, value, self).unwrap();
                            result.into()
                        })
                    }
                })
                .unwrap_or_else(|| scope.generate_ref_error(self)),
            ast::InlineExpression::NumberLiteral { value } => FluentValue::try_number(*value),
            ast::InlineExpression::TermReference {
                id,
                attribute,
                arguments,
            } => {
                let (_, resolved_named_args) = get_arguments(scope, arguments);

                scope.local_args = Some(resolved_named_args);

                let result = scope
                    .bundle
                    .get_entry_term(&id.name)
                    .and_then(|term| {
                        if let Some(attr) = attribute {
                            term.attributes
                                .iter()
                                .find(|a| a.id.name == attr.name)
                                .map(|attr| {
                                    let mut result = String::new();
                                    scope.track(&mut result, &attr.value, self).unwrap();
                                    FluentValue::String(result.into())
                                })
                        } else {
                            let mut result = String::new();
                            scope.track(&mut result, &term.value, self).unwrap();
                            Some(FluentValue::String(result.into()))
                        }
                    })
                    .unwrap_or_else(|| scope.generate_ref_error(self));
                scope.local_args = None;
                result
            }
            ast::InlineExpression::FunctionReference { id, arguments } => {
                let (resolved_positional_args, resolved_named_args) =
                    get_arguments(scope, arguments);

                let func = scope.bundle.get_entry_function(id.name);

                if let Some(func) = func {
                    let result = func(resolved_positional_args.as_slice(), &resolved_named_args);
                    if let FluentValue::Error = result {
                        self.resolve_error().into()
                    } else {
                        result
                    }
                } else {
                    scope.generate_ref_error(self)
                }
            }
            ast::InlineExpression::VariableReference { id } => {
                let args = scope.local_args.as_ref().or(scope.args);

                if let Some(arg) = args.and_then(|args| args.get(id.name)) {
                    arg.clone()
                } else {
                    if scope.local_args.is_none() {
                        scope.add_error(ResolverError::Reference(self.resolve_error()));
                    }
                    FluentValue::None
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn resolve_error(&self) -> String {
        match self {
            ast::InlineExpression::MessageReference {
                attribute: None, ..
            } => {
                let mut error = String::from("Unknown message: ");
                self.write_error(&mut error)
                    .expect("Failed to write to String.");
                error
            }
            ast::InlineExpression::MessageReference { .. } => {
                let mut error = String::from("Unknown attribute: ");
                self.write_error(&mut error)
                    .expect("Failed to write to String.");
                error
            }
            ast::InlineExpression::VariableReference { .. } => {
                let mut error = String::from("Unknown variable: ");
                self.write_error(&mut error)
                    .expect("Failed to write to String.");
                error
            }
            ast::InlineExpression::TermReference {
                attribute: None, ..
            } => {
                let mut error = String::from("Unknown term: ");
                self.write_error(&mut error)
                    .expect("Failed to write to String.");
                error
            }
            ast::InlineExpression::TermReference { .. } => {
                let mut error = String::from("Unknown attribute: ");
                self.write_error(&mut error)
                    .expect("Failed to write to String.");
                error
            }
            ast::InlineExpression::FunctionReference { .. } => {
                let mut error = String::from("Unknown function: ");
                self.write_error(&mut error)
                    .expect("Failed to write to String.");
                error
            }
            _ => unreachable!(),
        }
    }
}

fn get_arguments<'bundle, R, M: MemoizerKind>(
    scope: &mut Scope<'bundle, R, M>,
    arguments: &'bundle Option<ast::CallArguments<'bundle>>,
) -> (Vec<FluentValue<'bundle>>, FluentArgs<'bundle>)
where
    R: Borrow<FluentResource>,
{
    let mut resolved_positional_args = Vec::new();
    let mut resolved_named_args = FluentArgs::new();

    if let Some(ast::CallArguments { named, positional }) = arguments {
        for expression in positional {
            resolved_positional_args.push(expression.resolve(scope));
        }

        for arg in named {
            resolved_named_args.insert(arg.name.name, arg.value.resolve(scope));
        }
    }

    (resolved_positional_args, resolved_named_args)
}
