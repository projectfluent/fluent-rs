//! The `ResolveValue` trait resolves Fluent AST nodes to [`FluentValues`].
//!
//! This is an internal API used by [`FluentBundle`] to evaluate Messages, Attributes and other
//! AST nodes to [`FluentValues`] which can be then formatted to strings.
//!
//! [`FluentValues`]: ../types/enum.FluentValue.html
//! [`FluentBundle`]: ../bundle/struct.FluentBundle.html

mod errors;
mod expression;
mod inline_expression;
mod pattern;
mod scope;

pub use errors::ResolverError;
pub use scope::Scope;

use std::borrow::Borrow;
// use std::fmt::Write;
use std::fmt;

// use fluent_syntax::ast;
// use fluent_syntax::unicode::unescape_unicode;

// use crate::bundle::FluentArgs;
// use crate::entry::GetEntry;
use crate::memoizer::MemoizerKind;
use crate::resource::FluentResource;
// use crate::types::DisplayableNode;
use crate::types::FluentValue;

// const MAX_PLACEABLES: u8 = 100;

// fn generate_ref_error<'source, R, M>(
//     scope: &mut Scope<'source, R, M>,
//     node: DisplayableNode<'source>,
// ) -> FluentValue<'source>
// where
//     R: Borrow<FluentResource>,
// {
//     scope
//         .errors
//         .push(ResolverError::Reference(node.get_error()));
//     FluentValue::Error(node)
// }

// Converts an AST node to a `FluentValue`.
pub(crate) trait ResolveValue {
    fn resolve<'source, R, M: MemoizerKind>(
        &'source self,
        scope: &mut Scope<'source, R, M>,
    ) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>,
    {
        unimplemented!();
    }

    fn resolve_error(&self) -> String {
        unimplemented!();
    }
}

pub(crate) trait WriteValue {
    fn write<W, R, M: MemoizerKind>(&self, w: &mut W, scope: &mut Scope<R, M>) -> fmt::Result
    where
        W: fmt::Write,
        R: Borrow<FluentResource>;

    fn write_error<W>(&self, w: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        unimplemented!();
    }
}

// impl<'source> ResolveValue<'source> for ast::Pattern<'source> {
//     fn resolve<R, M: MemoizerKind>(
//         &'source self,
//         scope: &mut Scope<'source, R, M>,
//     ) -> FluentValue<'source>
//     where
//         R: Borrow<FluentResource>,
//     {
//         if scope.dirty {
//             return FluentValue::None;
//         }

//         if self.elements.len() == 1 {
//             return match self.elements[0] {
//                 ast::PatternElement::TextElement(s) => {
//                     if let Some(ref transform) = scope.bundle.transform {
//                         transform(s).into()
//                     } else {
//                         s.into()
//                     }
//                 }
//                 ast::PatternElement::Placeable(ref p) => scope.maybe_track(self, p),
//             };
//         }

//         let mut string = String::new();
//         for elem in &self.elements {
//             if scope.dirty {
//                 return FluentValue::None;
//             }

//             match elem {
//                 ast::PatternElement::TextElement(s) => {
//                     if let Some(ref transform) = scope.bundle.transform {
//                         string.push_str(&transform(s))
//                     } else {
//                         string.push_str(&s)
//                     }
//                 }
//                 ast::PatternElement::Placeable(p) => {
//                     scope.placeables += 1;
//                     if scope.placeables > MAX_PLACEABLES {
//                         scope.dirty = true;
//                         scope.errors.push(ResolverError::TooManyPlaceables);
//                         return FluentValue::None;
//                     }

//                     let needs_isolation = scope.bundle.use_isolating
//                         && match p {
//                             ast::Expression::InlineExpression(
//                                 ast::InlineExpression::MessageReference { .. },
//                             )
//                             | ast::Expression::InlineExpression(
//                                 ast::InlineExpression::TermReference { .. },
//                             )
//                             | ast::Expression::InlineExpression(
//                                 ast::InlineExpression::StringLiteral { .. },
//                             ) => false,
//                             _ => true,
//                         };
//                     if needs_isolation {
//                         string.write_char('\u{2068}').expect("Writing failed");
//                     }

//                     let result = scope.maybe_track(self, p);
//                     write!(string, "{}", result.as_string(scope)).expect("Writing failed");

//                     if needs_isolation {
//                         string.write_char('\u{2069}').expect("Writing failed");
//                     }
//                 }
//             }
//         }
//         string.into()
//     }
// }

// impl<'source> ResolveValue<'source> for ast::Expression<'source> {
//     fn resolve<R, M: MemoizerKind>(
//         &'source self,
//         scope: &mut Scope<'source, R, M>,
//     ) -> FluentValue<'source>
//     where
//         R: Borrow<FluentResource>,
//     {
//         match self {
//             ast::Expression::InlineExpression(exp) => exp.resolve(scope),
//             ast::Expression::SelectExpression { selector, variants } => {
//                 let selector = selector.resolve(scope);
//                 match selector {
//                     FluentValue::String(_) | FluentValue::Number(_) => {
//                         for variant in variants {
//                             let key = match variant.key {
//                                 ast::VariantKey::Identifier { name } => name.into(),
//                                 ast::VariantKey::NumberLiteral { value } => {
//                                     FluentValue::try_number(value)
//                                 }
//                             };
//                             if key.matches(&selector, &scope) {
//                                 return variant.value.resolve(scope);
//                             }
//                         }
//                     }
//                     _ => {}
//                 }

//                 for variant in variants {
//                     if variant.default {
//                         return variant.value.resolve(scope);
//                     }
//                 }
//                 scope.errors.push(ResolverError::MissingDefault);
//                 FluentValue::None
//             }
//         }
//     }
// }

// impl<'source> ResolveValue<'source> for ast::InlineExpression<'source> {
//     fn resolve<R, M: MemoizerKind>(
//         &'source self,
//         mut scope: &mut Scope<'source, R, M>,
//     ) -> FluentValue<'source>
//     where
//         R: Borrow<FluentResource>,
//     {
//         match self {
//             ast::InlineExpression::StringLiteral { value } => unescape_unicode(value).into(),
//             ast::InlineExpression::MessageReference { id, attribute } => scope
//                 .bundle
//                 .get_entry_message(&id.name)
//                 .and_then(|msg| {
//                     if let Some(attr) = attribute {
//                         msg.attributes
//                             .iter()
//                             .find(|a| a.id.name == attr.name)
//                             .map(|attr| scope.track(&attr.value, self.into()))
//                     } else {
//                         msg.value
//                             .as_ref()
//                             .map(|value| scope.track(value, self.into()))
//                     }
//                 })
//                 .unwrap_or_else(|| generate_ref_error(scope, self.into())),
//             ast::InlineExpression::NumberLiteral { value } => FluentValue::try_number(*value),
//             ast::InlineExpression::TermReference {
//                 id,
//                 attribute,
//                 arguments,
//             } => {
//                 let (_, resolved_named_args) = get_arguments(scope, arguments);

//                 scope.local_args = Some(resolved_named_args);

//                 let value = scope
//                     .bundle
//                     .get_entry_term(&id.name)
//                     .and_then(|term| {
//                         if let Some(attr) = attribute {
//                             term.attributes
//                                 .iter()
//                                 .find(|a| a.id.name == attr.name)
//                                 .map(|attr| scope.track(&attr.value, self.into()))
//                         } else {
//                             Some(scope.track(&term.value, self.into()))
//                         }
//                     })
//                     .unwrap_or_else(|| generate_ref_error(scope, self.into()));

//                 scope.local_args = None;
//                 value
//             }
//             ast::InlineExpression::FunctionReference { id, arguments } => {
//                 let (resolved_positional_args, resolved_named_args) =
//                     get_arguments(scope, arguments);

//                 let func = scope.bundle.get_entry_function(id.name);

//                 if let Some(func) = func {
//                     func(resolved_positional_args.as_slice(), &resolved_named_args)
//                 } else {
//                     generate_ref_error(scope, self.into())
//                 }
//             }
//             ast::InlineExpression::VariableReference { id } => {
//                 let args = scope.local_args.as_ref().or(scope.args);

//                 if let Some(arg) = args.and_then(|args| args.get(id.name)) {
//                     arg.clone()
//                 } else {
//                     let entry: DisplayableNode = self.into();
//                     if scope.local_args.is_none() {
//                         scope
//                             .errors
//                             .push(ResolverError::Reference(entry.get_error()));
//                     }
//                     FluentValue::Error(entry)
//                 }
//             }
//             ast::InlineExpression::Placeable { expression } => expression.resolve(scope),
//         }
//     }
// }

// fn get_arguments<'bundle, R, M: MemoizerKind>(
//     scope: &mut Scope<'bundle, R, M>,
//     arguments: &'bundle Option<ast::CallArguments<'bundle>>,
// ) -> (Vec<FluentValue<'bundle>>, FluentArgs<'bundle>)
// where
//     R: Borrow<FluentResource>,
// {
//     let mut resolved_positional_args = Vec::new();
//     let mut resolved_named_args = FluentArgs::new();

//     if let Some(ast::CallArguments { named, positional }) = arguments {
//         for expression in positional {
//             resolved_positional_args.push(expression.resolve(scope));
//         }

//         for arg in named {
//             resolved_named_args.insert(arg.name.name, arg.value.resolve(scope));
//         }
//     }

//     (resolved_positional_args, resolved_named_args)
// }
