//! The `ResolveValue` trait resolves Fluent AST nodes to [`FluentValues`].
//!
//! This is an internal API used by [`FluentBundle`] to evaluate Messages, Attributes and other
//! AST nodes to [`FluentValues`] which can be then formatted to strings.
//!
//! [`FluentValues`]: ../types/enum.FluentValue.html
//! [`FluentBundle`]: ../bundle/struct.FluentBundle.html

use std::borrow::Borrow;
use std::fmt::Write;

use fluent_syntax::ast;
use fluent_syntax::unicode::unescape_unicode;

use crate::bundle::{FluentArgs, FluentBundle};
use crate::entry::GetEntry;
use crate::resource::FluentResource;
use crate::types::DisplayableNode;
use crate::types::FluentValue;

#[derive(Debug, PartialEq, Clone)]
pub enum ResolverError {
    Reference(String),
    MissingDefault,
    Cyclic,
}

/// State for a single `ResolveValue::to_value` call.
pub struct Scope<'bundle, R: Borrow<FluentResource>, W: Write> {
    /// The current `FluentBundle` instance.
    pub bundle: &'bundle FluentBundle<R>,
    /// The current arguments passed by the developer.
    args: Option<&'bundle FluentArgs<'bundle>>,
    /// Local args
    local_args: Option<FluentArgs<'bundle>>,
    /// Tracks hashes to prevent infinite recursion.
    travelled: smallvec::SmallVec<[&'bundle ast::Pattern<'bundle>; 2]>,
    /// Track errors accumulated during resolving.
    pub errors: Vec<ResolverError>,
    pub writer: W,
}

impl<'bundle, R: Borrow<FluentResource>, W: Write> Scope<'bundle, R, W> {
    pub fn new(
        bundle: &'bundle FluentBundle<R>,
        args: Option<&'bundle FluentArgs>,
        writer: W,
    ) -> Self {
        Scope {
            bundle,
            args,
            local_args: None,
            travelled: Default::default(),
            errors: vec![],
            writer,
        }
    }

    // This method allows us to lazily add Pattern on the stack,
    // only if the Pattern::resolve has been called on an empty stack.
    //
    // This is the case when pattern is called from Bundle and it
    // allows us to fast-path simple resolutions, and only use the stack
    // for placeables.
    pub fn maybe_track(
        &mut self,
        pattern: &'bundle ast::Pattern,
        placeable: &'bundle ast::Expression,
    ) -> FluentValue<'bundle> {
        if self.travelled.is_empty() {
            self.travelled.push(pattern);
        }
        placeable.resolve(self)
    }
    pub fn maybe_track_write(
        &mut self,
        pattern: &'bundle ast::Pattern,
        placeable: &'bundle ast::Expression,
    ) -> Result<(), std::fmt::Error> {
        if self.travelled.is_empty() {
            self.travelled.push(pattern);
        }
        placeable.fmt(self)
    }

    pub fn track(
        &mut self,
        pattern: &'bundle ast::Pattern,
        entry: DisplayableNode<'bundle>,
    ) -> FluentValue<'bundle> {
        if self.travelled.contains(&pattern) {
            self.errors.push(ResolverError::Cyclic);
            FluentValue::Error(entry)
        } else {
            self.travelled.push(pattern);
            let result = pattern.resolve(self);
            self.travelled.pop();
            result
        }
    }

    pub fn track_write(
        &mut self,
        pattern: &'bundle ast::Pattern,
        entry: DisplayableNode<'bundle>,
    ) -> Result<(), std::fmt::Error> {
        if self.travelled.contains(&pattern) {
            self.errors.push(ResolverError::Cyclic);
            write!(self.writer, "{{{}}}", entry)
        } else {
            self.travelled.push(pattern);
            let result = pattern.fmt(self);
            self.travelled.pop();
            result
        }
    }
}

fn generate_ref_error<'source, R, W: Write>(
    scope: &mut Scope<'source, R, W>,
    node: DisplayableNode<'source>,
) -> FluentValue<'source>
where
    R: Borrow<FluentResource>,
{
    scope
        .errors
        .push(ResolverError::Reference(node.get_error()));
    FluentValue::Error(node)
}

fn generate_ref_error_write<'source, R, W: Write>(
    scope: &mut Scope<'source, R, W>,
    node: DisplayableNode<'source>,
) -> Result<(), std::fmt::Error>
where
    R: Borrow<FluentResource>,
{
    scope
        .errors
        .push(ResolverError::Reference(node.get_error()));
    write!(scope.writer, "{{{}}}", node)
}

// Converts an AST node to a `FluentValue`.
pub trait ResolveValue<'source> {
    fn resolve<R, W: Write>(
        &'source self,
        scope: &mut Scope<'source, R, W>,
    ) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>;

    fn fmt<R, W: Write>(
        &'source self,
        scope: &mut Scope<'source, R, W>,
    ) -> Result<(), std::fmt::Error>
    where
        R: Borrow<FluentResource>;
}

impl<'source> ResolveValue<'source> for ast::Pattern<'source> {
    fn resolve<R, W: Write>(&'source self, scope: &mut Scope<'source, R, W>) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>,
    {
        if self.elements.len() == 1 {
            return match self.elements[0] {
                ast::PatternElement::TextElement(s) => {
                    if let Some(ref transform) = scope.bundle.transform {
                        FluentValue::String(transform(s))
                    } else {
                        s.into()
                    }
                }
                ast::PatternElement::Placeable(ref p) => scope.maybe_track(self, p),
            };
        }

        let mut string = String::new();
        for elem in &self.elements {
            match elem {
                ast::PatternElement::TextElement(s) => {
                    if let Some(ref transform) = scope.bundle.transform {
                        string.push_str(&transform(s))
                    } else {
                        string.push_str(&s)
                    }
                }
                ast::PatternElement::Placeable(p) => {
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
                        string.write_char('\u{2068}').expect("Writing failed");
                    }
                    let result = scope.maybe_track(self, p);
                    write!(string, "{}", result).expect("Writing failed");
                    if needs_isolation {
                        string.write_char('\u{2069}').expect("Writing failed");
                    }
                }
            }
        }
        FluentValue::String(string.into())
    }

    fn fmt<R, W: Write>(
        &'source self,
        scope: &mut Scope<'source, R, W>,
    ) -> Result<(), std::fmt::Error>
    where
        R: Borrow<FluentResource>,
    {
        if self.elements.len() == 1 {
            return match self.elements[0] {
                ast::PatternElement::TextElement(s) => {
                    if let Some(ref transform) = scope.bundle.transform {
                        scope.writer.write_str(&transform(s))
                    } else {
                        scope.writer.write_str(s)
                    }
                }
                ast::PatternElement::Placeable(ref p) => scope.maybe_track_write(self, p),
            };
        }

        for elem in &self.elements {
            match elem {
                ast::PatternElement::TextElement(s) => {
                    if let Some(ref transform) = scope.bundle.transform {
                        scope.writer.write_str(&transform(s))?
                    } else {
                        scope.writer.write_str(s)?
                    }
                }
                ast::PatternElement::Placeable(p) => {
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
                        scope.writer.write_char('\u{2068}')?;
                    }
                    scope.maybe_track_write(self, p)?;
                    if needs_isolation {
                        scope.writer.write_char('\u{2069}')?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl<'source> ResolveValue<'source> for ast::Expression<'source> {
    fn resolve<R, W: Write>(&'source self, scope: &mut Scope<'source, R, W>) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>,
    {
        match self {
            ast::Expression::InlineExpression(exp) => exp.resolve(scope),
            ast::Expression::SelectExpression { selector, variants } => {
                let selector = selector.resolve(scope);
                match selector {
                    FluentValue::String(_) | FluentValue::Number(_) => {
                        for variant in variants {
                            let key = match variant.key {
                                ast::VariantKey::Identifier { name } => {
                                    FluentValue::String(name.into())
                                }
                                ast::VariantKey::NumberLiteral { value } => {
                                    FluentValue::into_number(value)
                                }
                            };
                            if key.matches(&selector, &scope) {
                                return variant.value.resolve(scope);
                            }
                        }
                    }
                    _ => {}
                }

                for variant in variants {
                    if variant.default {
                        return variant.value.resolve(scope);
                    }
                }
                scope.errors.push(ResolverError::MissingDefault);
                FluentValue::None
            }
        }
    }
    fn fmt<R, W: Write>(
        &'source self,
        scope: &mut Scope<'source, R, W>,
    ) -> Result<(), std::fmt::Error>
    where
        R: Borrow<FluentResource>,
    {
        match self {
            ast::Expression::InlineExpression(exp) => exp.fmt(scope),
            ast::Expression::SelectExpression { selector, variants } => {
                let selector = selector.resolve(scope);
                match selector {
                    FluentValue::String(_) | FluentValue::Number(_) => {
                        for variant in variants {
                            let key = match variant.key {
                                ast::VariantKey::Identifier { name } => {
                                    FluentValue::String(name.into())
                                }
                                ast::VariantKey::NumberLiteral { value } => {
                                    FluentValue::into_number(value)
                                }
                            };
                            if key.matches(&selector, &scope) {
                                return variant.value.fmt(scope);
                            }
                        }
                    }
                    _ => {}
                }

                for variant in variants {
                    if variant.default {
                        return variant.value.fmt(scope);
                    }
                }
                scope.errors.push(ResolverError::MissingDefault);
                Ok(())
            }
        }
    }
}

impl<'source> ResolveValue<'source> for ast::InlineExpression<'source> {
    fn resolve<R, W: Write>(
        &'source self,
        mut scope: &mut Scope<'source, R, W>,
    ) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>,
    {
        match self {
            ast::InlineExpression::StringLiteral { value } => {
                FluentValue::String(unescape_unicode(value))
            }
            ast::InlineExpression::MessageReference { id, attribute } => scope
                .bundle
                .get_entry_message(&id.name)
                .and_then(|msg| {
                    if let Some(attr) = attribute {
                        msg.attributes
                            .iter()
                            .find(|a| a.id.name == attr.name)
                            .map(|attr| scope.track(&attr.value, self.into()))
                    } else {
                        msg.value
                            .as_ref()
                            .map(|value| scope.track(value, self.into()))
                    }
                })
                .unwrap_or_else(|| generate_ref_error(scope, self.into())),
            ast::InlineExpression::NumberLiteral { value } => FluentValue::into_number(*value),
            ast::InlineExpression::TermReference {
                id,
                attribute,
                arguments,
            } => {
                let (_, resolved_named_args) = get_arguments(scope, arguments);

                scope.local_args = Some(resolved_named_args);

                let value = scope
                    .bundle
                    .get_entry_term(&id.name)
                    .and_then(|term| {
                        if let Some(attr) = attribute {
                            term.attributes
                                .iter()
                                .find(|a| a.id.name == attr.name)
                                .map(|attr| scope.track(&attr.value, self.into()))
                        } else {
                            Some(scope.track(&term.value, self.into()))
                        }
                    })
                    .unwrap_or_else(|| generate_ref_error(scope, self.into()));

                scope.local_args = None;
                value
            }
            ast::InlineExpression::FunctionReference { id, arguments } => {
                let (resolved_positional_args, resolved_named_args) =
                    get_arguments(scope, arguments);

                let func = scope.bundle.get_entry_function(id.name);

                if let Some(func) = func {
                    func(resolved_positional_args.as_slice(), &resolved_named_args)
                } else {
                    generate_ref_error(scope, self.into())
                }
            }
            ast::InlineExpression::VariableReference { id } => {
                let args = scope.local_args.as_ref().or(scope.args);

                if let Some(arg) = args.and_then(|args| args.get(id.name)) {
                    arg.clone()
                } else {
                    let entry: DisplayableNode = self.into();
                    if scope.local_args.is_none() {
                        scope
                            .errors
                            .push(ResolverError::Reference(entry.get_error()));
                    }
                    FluentValue::Error(entry)
                }
            }
            ast::InlineExpression::Placeable { expression } => expression.resolve(scope),
        }
    }
    fn fmt<R, W: Write>(
        &'source self,
        mut scope: &mut Scope<'source, R, W>,
    ) -> Result<(), std::fmt::Error>
    where
        R: Borrow<FluentResource>,
    {
        match self {
            ast::InlineExpression::StringLiteral { value } => {
                scope.writer.write_str(&unescape_unicode(value))
            }
            ast::InlineExpression::MessageReference { id, attribute } => scope
                .bundle
                .get_entry_message(&id.name)
                .and_then(|msg| {
                    if let Some(attr) = attribute {
                        msg.attributes
                            .iter()
                            .find(|a| a.id.name == attr.name)
                            .map(|attr| scope.track_write(&attr.value, self.into()))
                    } else {
                        msg.value
                            .as_ref()
                            .map(|value| scope.track_write(value, self.into()))
                    }
                })
                .unwrap_or_else(|| generate_ref_error_write(scope, self.into())),
            ast::InlineExpression::NumberLiteral { value } => scope.writer.write_str(value),
            ast::InlineExpression::TermReference {
                id,
                attribute,
                arguments,
            } => {
                let (_, resolved_named_args) = get_arguments(scope, arguments);

                scope.local_args = Some(resolved_named_args);

                let value = scope
                    .bundle
                    .get_entry_term(&id.name)
                    .and_then(|term| {
                        if let Some(attr) = attribute {
                            term.attributes
                                .iter()
                                .find(|a| a.id.name == attr.name)
                                .map(|attr| scope.track_write(&attr.value, self.into()))
                        } else {
                            Some(scope.track_write(&term.value, self.into()))
                        }
                    })
                    .unwrap_or_else(|| generate_ref_error_write(scope, self.into()));

                scope.local_args = None;
                value
            }
            ast::InlineExpression::FunctionReference { id, arguments } => {
                let (resolved_positional_args, resolved_named_args) =
                    get_arguments(scope, arguments);

                let func = scope.bundle.get_entry_function(id.name);

                if let Some(func) = func {
                    let val = func(resolved_positional_args.as_slice(), &resolved_named_args);
                    write!(scope.writer, "{}", val)
                } else {
                    generate_ref_error_write(scope, self.into())
                }
            }
            ast::InlineExpression::VariableReference { id } => {
                let args = scope.local_args.as_ref().or(scope.args);

                if let Some(arg) = args.and_then(|args| args.get(id.name)) {
                    write!(scope.writer, "{}", arg)
                } else {
                    let entry: DisplayableNode = self.into();
                    if scope.local_args.is_none() {
                        scope
                            .errors
                            .push(ResolverError::Reference(entry.get_error()));
                    }
                    write!(scope.writer, "{{{}}}", entry)
                }
            }
            ast::InlineExpression::Placeable { expression } => expression.fmt(scope),
        }
    }
}

fn get_arguments<'bundle, R, W: Write>(
    scope: &mut Scope<'bundle, R, W>,
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
