//! The `ResolveValue` trait resolves Fluent AST nodes to [`FluentValues`].
//!
//! This is an internal API used by [`FluentBundle`] to evaluate Messages, Attributes and other
//! AST nodes to [`FluentValues`] which can be then formatted to strings.
//!
//! [`FluentValues`]: ../types/enum.FluentValue.html
//! [`FluentBundle`]: ../bundle/struct.FluentBundle.html

use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use fluent_syntax::ast;
use fluent_syntax::unicode::unescape_unicode;

use crate::bundle::FluentBundle;
use crate::entry::GetEntry;
use crate::resource::FluentResource;
use crate::types::DisplayableNode;
use crate::types::FluentValue;

#[derive(Debug, PartialEq, Clone)]
pub enum ResolverError {
    Reference(String),
    MissingDefault,
    Argument(String),
    Value,
    Cyclic,
}

/// State for a single `ResolveValue::to_value` call.
pub struct Scope<'bundle, R: Borrow<FluentResource>> {
    /// The current `FluentBundle` instance.
    pub bundle: &'bundle FluentBundle<'bundle, R>,
    /// The current arguments passed by the developer.
    pub args: Option<&'bundle HashMap<&'bundle str, FluentValue<'bundle>>>,
    /// Local args
    pub local_args: Option<HashMap<&'bundle str, FluentValue<'bundle>>>,
    /// Tracks hashes to prevent infinite recursion.
    pub travelled: RefCell<Vec<u64>>,
    /// Track errors accumulated during resolving.
    pub errors: Vec<ResolverError>,
}

impl<'bundle, R: Borrow<FluentResource>> Scope<'bundle, R> {
    pub fn new(
        bundle: &'bundle FluentBundle<R>,
        args: Option<&'bundle HashMap<&str, FluentValue>>,
    ) -> Self {
        Scope {
            bundle,
            args,
            local_args: None,
            travelled: RefCell::new(Vec::new()),
            errors: vec![],
        }
    }

    pub fn track<F>(
        &mut self,
        entry: DisplayableNode<'bundle>,
        mut action: F,
    ) -> FluentValue<'bundle>
    where
        F: FnMut(&mut Scope<'bundle, R>) -> FluentValue<'bundle>,
    {
        let mut hasher = DefaultHasher::new();
        (entry.id, entry.attribute).hash(&mut hasher);
        entry.id.hash(&mut hasher);
        if let Some(attr) = entry.attribute {
            attr.hash(&mut hasher);
        }
        let hash = hasher.finish();

        if self.travelled.borrow().contains(&hash) {
            self.errors.push(ResolverError::Cyclic);
            FluentValue::Error(entry)
        } else {
            self.travelled.borrow_mut().push(hash);
            let result = action(self);
            self.travelled.borrow_mut().pop();
            result
        }
    }

    fn maybe_resolve_attribute(
        &mut self,
        attributes: &[ast::Attribute<'bundle>],
        entry: DisplayableNode<'bundle>,
        name: &str,
    ) -> Option<FluentValue<'bundle>> {
        attributes
            .iter()
            .find(|attr| attr.id.name == name)
            .map(|attr| self.track(entry, |env| attr.value.resolve(env)))
    }
}

pub fn resolve_value_for_entry<'source, R>(
    value: &ast::Pattern<'source>,
    entry: DisplayableNode<'source>,
    env: &mut Scope<'source, R>,
) -> FluentValue<'source>
where
    R: Borrow<FluentResource>,
{
    if value.elements.len() == 1 {
        return match value.elements[0] {
            ast::PatternElement::TextElement(s) => FluentValue::String(s.into()),
            ast::PatternElement::Placeable(ref p) => env.track(entry.clone(), |env| p.resolve(env)),
        };
    }

    let mut string = String::new();
    for elem in &value.elements {
        match elem {
            ast::PatternElement::TextElement(s) => {
                string.push_str(&s);
            }
            ast::PatternElement::Placeable(p) => {
                let result = env.track(entry.clone(), |env| p.resolve(env));
                string.push_str(&result.to_string());
            }
        }
    }
    FluentValue::String(string.into())
}

fn generate_ref_error<'source, R>(
    env: &mut Scope<'source, R>,
    node: DisplayableNode<'source>,
) -> FluentValue<'source>
where
    R: Borrow<FluentResource>,
{
    env.errors.push(ResolverError::Reference(node.get_error()));
    FluentValue::Error(node)
}

// Converts an AST node to a `FluentValue`.
pub trait ResolveValue<'source> {
    fn resolve<R>(&self, env: &mut Scope<'source, R>) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>;
}

impl<'source> ResolveValue<'source> for ast::Term<'source> {
    fn resolve<R>(&self, env: &mut Scope<'source, R>) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>,
    {
        resolve_value_for_entry(&self.value, self.into(), env)
    }
}

impl<'source> ResolveValue<'source> for ast::Pattern<'source> {
    fn resolve<R>(&self, env: &mut Scope<'source, R>) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>,
    {
        if self.elements.len() == 1 {
            return match self.elements[0] {
                ast::PatternElement::TextElement(s) => FluentValue::String(s.into()),
                ast::PatternElement::Placeable(ref p) => p.resolve(env),
            };
        }

        let mut string = String::new();
        for elem in &self.elements {
            match elem {
                ast::PatternElement::TextElement(s) => {
                    string.push_str(&s);
                }
                ast::PatternElement::Placeable(p) => {
                    let result = p.resolve(env).to_string();
                    string.push_str(&result);
                }
            }
        }
        FluentValue::String(string.into())
    }
}

impl<'source> ResolveValue<'source> for ast::Expression<'source> {
    fn resolve<R>(&self, env: &mut Scope<'source, R>) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>,
    {
        match self {
            ast::Expression::InlineExpression(exp) => exp.resolve(env),
            ast::Expression::SelectExpression {
                selector,
                ref variants,
            } => {
                let selector = selector.resolve(env);
                match selector {
                    FluentValue::String(_) | FluentValue::Number(_) => {
                        for variant in variants {
                            match variant.key {
                                ast::VariantKey::Identifier { name } => {
                                    let key = FluentValue::String(name.into());
                                    if key.matches(&selector, &env) {
                                        return variant.value.resolve(env);
                                    }
                                }
                                ast::VariantKey::NumberLiteral { value } => {
                                    let key = FluentValue::into_number(value);
                                    if key.matches(&selector, &env) {
                                        return variant.value.resolve(env);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }

                for variant in variants {
                    if variant.default {
                        return variant.value.resolve(env);
                    }
                }
                env.errors.push(ResolverError::MissingDefault);
                FluentValue::None()
            }
        }
    }
}

impl<'source> ResolveValue<'source> for ast::InlineExpression<'source> {
    fn resolve<R>(&self, mut env: &mut Scope<'source, R>) -> FluentValue<'source>
    where
        R: Borrow<FluentResource>,
    {
        match self {
            ast::InlineExpression::StringLiteral { value } => {
                FluentValue::String(unescape_unicode(value))
            }
            ast::InlineExpression::MessageReference { id, attribute } => {
                let msg = env.bundle.get_message(&id.name);

                if let Some(msg) = msg {
                    if let Some(attr) = attribute {
                        env.maybe_resolve_attribute(&msg.attributes, self.into(), attr.name)
                            .unwrap_or_else(|| generate_ref_error(env, self.into()))
                    } else if let Some(value) = msg.value.as_ref() {
                        env.track(self.into(), |env| value.resolve(env))
                    } else {
                        generate_ref_error(env, self.into())
                    }
                } else {
                    generate_ref_error(env, self.into())
                }
            }
            ast::InlineExpression::NumberLiteral { value } => FluentValue::into_number(*value),
            ast::InlineExpression::TermReference {
                id,
                attribute,
                arguments,
            } => {
                let term = env.bundle.get_term(&id.name);

                let (_, resolved_named_args) = get_arguments(env, arguments);

                env.local_args = Some(resolved_named_args);

                let value = if let Some(term) = term {
                    if let Some(attr) = attribute {
                        env.maybe_resolve_attribute(&term.attributes, self.into(), attr.name)
                            .unwrap_or_else(|| generate_ref_error(env, self.into()))
                    } else {
                        term.resolve(&mut env)
                    }
                } else {
                    generate_ref_error(env, self.into())
                };
                env.local_args = None;
                value
            }
            ast::InlineExpression::FunctionReference { id, arguments } => {
                let (resolved_positional_args, resolved_named_args) = get_arguments(env, arguments);

                let func = env.bundle.get_function(id.name);

                if let Some(func) = func {
                    func(resolved_positional_args.as_slice(), &resolved_named_args)
                } else {
                    generate_ref_error(env, self.into())
                }
            }
            ast::InlineExpression::VariableReference { id } => {
                let arg = if let Some(args) = &env.local_args {
                    args.get(&id.name)
                } else {
                    env.args.and_then(|args| args.get(&id.name))
                };
                if let Some(arg) = arg {
                    arg.clone()
                } else {
                    let displayable_node: DisplayableNode = self.into();
                    if env.local_args.is_none() {
                        env.errors
                            .push(ResolverError::Reference(displayable_node.get_error()));
                    }
                    FluentValue::Error(displayable_node)
                }
            }
            ast::InlineExpression::Placeable { expression } => expression.resolve(env),
        }
    }
}

fn get_arguments<'bundle, R>(
    env: &mut Scope<'bundle, R>,
    arguments: &Option<ast::CallArguments<'bundle>>,
) -> (
    Vec<FluentValue<'bundle>>,
    HashMap<&'bundle str, FluentValue<'bundle>>,
)
where
    R: Borrow<FluentResource>,
{
    let mut resolved_positional_args = Vec::new();
    let mut resolved_named_args = HashMap::new();

    if let Some(ast::CallArguments { named, positional }) = arguments {
        for expression in positional {
            resolved_positional_args.push(expression.resolve(env));
        }

        for arg in named {
            resolved_named_args.insert(arg.name.name, arg.value.resolve(env));
        }
    }

    (resolved_positional_args, resolved_named_args)
}
