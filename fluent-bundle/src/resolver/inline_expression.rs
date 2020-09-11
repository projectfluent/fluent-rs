use super::scope::Scope;
use super::WriteValue;

use std::borrow::Borrow;
use std::fmt;

use fluent_syntax::ast;

use crate::memoizer::MemoizerKind;
use crate::resource::FluentResource;

impl<'p> WriteValue for ast::InlineExpression<'p> {
    fn write<W, R, M: MemoizerKind>(&self, w: &mut W, scope: &mut Scope<R, M>) -> fmt::Result
    where
        W: fmt::Write,
        R: Borrow<FluentResource>,
    {
        // match self {
        //     ast::InlineExpression::StringLiteral { value } => unescape_unicode(value).into(),
        //     ast::InlineExpression::MessageReference { id, attribute } => scope
        //         .bundle
        //         .get_entry_message(&id.name)
        //         .and_then(|msg| {
        //             if let Some(attr) = attribute {
        //                 msg.attributes
        //                     .iter()
        //                     .find(|a| a.id.name == attr.name)
        //                     .map(|attr| scope.track(&attr.value, self.into()))
        //             } else {
        //                 msg.value
        //                     .as_ref()
        //                     .map(|value| scope.track(value, self.into()))
        //             }
        //         })
        //         .unwrap_or_else(|| generate_ref_error(scope, self.into())),
        //     ast::InlineExpression::NumberLiteral { value } => FluentValue::try_number(*value),
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
        //     ast::InlineExpression::VariableReference { id } => {
        //         let args = scope.local_args.as_ref().or(scope.args);

        //         if let Some(arg) = args.and_then(|args| args.get(id.name)) {
        //             arg.clone()
        //         } else {
        //             let entry: DisplayableNode = self.into();
        //             if scope.local_args.is_none() {
        //                 scope
        //                     .errors
        //                     .push(ResolverError::Reference(entry.get_error()));
        //             }
        //             FluentValue::Error(entry)
        //         }
        //     }
        //     ast::InlineExpression::Placeable { expression } => expression.resolve(scope),
        // }
        Ok(())
    }
}
