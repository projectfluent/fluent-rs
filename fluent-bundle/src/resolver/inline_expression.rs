use super::scope::Scope;
use super::{ResolveValue, WriteValue};

use std::borrow::Borrow;
use std::fmt;

use fluent_syntax::ast;
use fluent_syntax::unicode::unescape_unicode;

use crate::memoizer::MemoizerKind;
use crate::resource::FluentResource;

impl<'p> WriteValue for ast::InlineExpression<'p> {
    fn write<W, R, M: MemoizerKind>(&self, w: &mut W, scope: &mut Scope<R, M>) -> fmt::Result
    where
        W: fmt::Write,
        R: Borrow<FluentResource>,
    {
        match self {
            ast::InlineExpression::StringLiteral { value } => unescape_unicode(w, value),
            ast::InlineExpression::MessageReference { id, attribute } => scope
                .bundle
                .get_message(&id.name)
                .and_then(|msg| {
                    if let Some(attr) = attribute {
                        msg.attributes
                            .get(attr.name)
                            .map(|pattern| scope.track(w, pattern, self))
                    } else {
                        msg.value.as_ref().map(|value| scope.track(w, value, self))
                    }
                })
                .unwrap_or_else(|| scope.generate_ref_error(w, self)),
            ast::InlineExpression::NumberLiteral { value } => w.write_str(value),
            //     ast::InlineExpression::TermReference {
            //         id,
            //         attribute,
            //         arguments,
            //     } => {
            //         let (_, resolved_named_args) = get_arguments(scope, arguments);

            //         scope.local_args = Some(resolved_named_args);

            //         let value = scope
            //             .bundle
            //             .get_entry_term(&id.name)
            //             .and_then(|term| {
            //                 if let Some(attr) = attribute {
            //                     term.attributes
            //                         .iter()
            //                         .find(|a| a.id.name == attr.name)
            //                         .map(|attr| scope.track(&attr.value, self.into()))
            //                 } else {
            //                     Some(scope.track(&term.value, self.into()))
            //                 }
            //             })
            //             .unwrap_or_else(|| generate_ref_error(scope, self.into()));

            //         scope.local_args = None;
            //         value
            //     }
            //     ast::InlineExpression::FunctionReference { id, arguments } => {
            //         let (resolved_positional_args, resolved_named_args) =
            //             get_arguments(scope, arguments);

            //         let func = scope.bundle.get_entry_function(id.name);

            //         if let Some(func) = func {
            //             func(resolved_positional_args.as_slice(), &resolved_named_args)
            //         } else {
            //             generate_ref_error(scope, self.into())
            //         }
            //     }
            // ast::InlineExpression::VariableReference { id } => {
            //     let args = scope.local_args.as_ref().or(scope.args);

            //     if let Some(arg) = args.and_then(|args| args.get(id.name)) {
            //         arg.clone()
            //     } else {
            //         let entry: DisplayableNode = self.into();
            //         if scope.local_args.is_none() {
            //             scope
            //                 .errors
            //                 .push(ResolverError::Reference(entry.get_error()));
            //         }
            //         FluentValue::Error(entry)
            //     }
            // }
            ast::InlineExpression::Placeable { expression } => expression.write(w, scope),
            _ => {
                unimplemented!();
            }
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
            _ => unreachable!(),
        }
    }
}

impl<'p> ResolveValue for ast::InlineExpression<'p> {
    fn resolve_error(&self) -> String {
        match self {
            ast::InlineExpression::MessageReference { .. } => {
                let mut error = String::from("Unknown message: ");
                self.write_error(&mut error)
                    .expect("Failed to write to String.");
                error
            }
            _ => unreachable!(),
        }
    }
}
