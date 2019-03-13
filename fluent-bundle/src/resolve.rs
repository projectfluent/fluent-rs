//! The `ResolveValue` trait resolves Fluent AST nodes to [`FluentValues`].
//!
//! This is an internal API used by [`FluentBundle`] to evaluate Messages, Attributes and other
//! AST nodes to [`FluentValues`] which can be then formatted to strings.
//!
//! [`FluentValues`]: ../types/enum.FluentValue.html
//! [`FluentBundle`]: ../bundle/struct.FluentBundle.html

use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::bundle::FluentBundle;
use super::entry::GetEntry;
use super::types::FluentValue;
use fluent_syntax::ast;
use fluent_syntax::unicode::unescape_unicode;

#[derive(Debug, PartialEq, Clone)]
pub enum ResolverError {
    None,
    Value,
    Cyclic,
}

/// State for a single `ResolveValue::to_value` call.
pub struct Env<'env> {
    /// The current `FluentBundle` instance.
    pub bundle: &'env FluentBundle<'env>,
    /// The current arguments passed by the developer.
    pub args: Option<&'env HashMap<&'env str, FluentValue<'env>>>,
    /// Local args
    pub local_args: Option<HashMap<&'env str, FluentValue<'env>>>,
    /// Tracks hashes to prevent infinite recursion.
    pub travelled: RefCell<Vec<u64>>,
    /// Track errors accumulated during resolving.
    pub errors: Vec<ResolverError>,
}

impl<'env> Env<'env> {
    pub fn new(
        bundle: &'env FluentBundle<'env>,
        args: Option<&'env HashMap<&str, FluentValue>>,
    ) -> Self {
        Env {
            bundle,
            args,
            local_args: None,
            travelled: RefCell::new(Vec::new()),
            errors: vec![],
        }
    }

    pub fn track<F>(
        &mut self,
        node_id: &'env str,
        entry_id: Option<&'env str>,
        mut action: F,
    ) -> FluentValue<'env>
    where
        F: FnMut(&mut Env<'env>) -> FluentValue<'env>,
    {
        let mut hasher = DefaultHasher::new();
        node_id.hash(&mut hasher);
        if let Some(entry_id) = entry_id {
            entry_id.hash(&mut hasher);
        }
        let hash = hasher.finish();

        if self.travelled.borrow().contains(&hash) {
            self.errors.push(ResolverError::Cyclic);
            let label = if let Some(entry_id) = entry_id {
                format!("{}.{}", entry_id, node_id).into()
            } else {
                node_id.into()
            };
            FluentValue::None(Some(label))
        } else {
            self.travelled.borrow_mut().push(hash);
            let result = action(self);
            self.travelled.borrow_mut().pop();
            result
        }
    }
}

// Converts an AST node to a `FluentValue`.
pub trait ResolveValue<'source> {
    fn resolve(&self, env: &mut Env<'source>) -> FluentValue<'source>;
}

pub trait ResolveValueForEntry<'source> {
    fn resolve_for_entry(
        &self,
        name: &'source str,
        entry_id: Option<&'source str>,
        env: &mut Env<'source>,
    ) -> FluentValue<'source>;
}

impl<'source> ResolveValue<'source> for ast::Message<'source> {
    fn resolve(&self, env: &mut Env<'source>) -> FluentValue<'source> {
        if let Some(value) = &self.value {
            env.track(&self.id.name, None, |env| value.resolve(env))
        } else {
            env.errors.push(ResolverError::None);
            FluentValue::None(Some(self.id.name.into()))
        }
    }
}

fn maybe_resolve_attribute<'source>(
    env: &mut Env<'source>,
    attributes: &[ast::Attribute<'source>],
    entry_name: &'source str,
    name: &str,
) -> Option<FluentValue<'source>> {
    attributes
        .iter()
        .find(|attr| attr.id.name == name)
        .map(|attr| {
            env.track(attr.id.name, Some(entry_name), |env| {
                attr.value.resolve(env)
            })
        })
}

impl<'source> ResolveValue<'source> for ast::Term<'source> {
    fn resolve(&self, env: &mut Env<'source>) -> FluentValue<'source> {
        env.track(&self.id.name, None, |env| self.value.resolve(env))
    }
}

impl<'source> ResolveValueForEntry<'source> for ast::Pattern<'source> {
    fn resolve_for_entry(
        &self,
        name: &'source str,
        entry_id: Option<&'source str>,
        env: &mut Env<'source>,
    ) -> FluentValue<'source> {
        if self.elements.len() == 1 {
            if let ast::PatternElement::TextElement(s) = self.elements[0] {
                return FluentValue::String(s.into());
            }
        }

        let mut string = String::new();
        for elem in &self.elements {
            match elem {
                ast::PatternElement::TextElement(s) => {
                    string.push_str(&s);
                }
                ast::PatternElement::Placeable(p) => {
                    let result = env.track(&name, entry_id, |env| p.resolve(env));
                    string.push_str(&result.to_string());
                }
            }
        }
        FluentValue::String(string.into())
    }
}

impl<'source> ResolveValue<'source> for ast::Pattern<'source> {
    fn resolve(&self, env: &mut Env<'source>) -> FluentValue<'source> {
        if self.elements.len() == 1 {
            if let ast::PatternElement::TextElement(s) = self.elements[0] {
                return FluentValue::String(s.into());
            }
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
    fn resolve(&self, env: &mut Env<'source>) -> FluentValue<'source> {
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
                FluentValue::None(None)
            }
        }
    }
}

fn format_reference_error<'source>(expr: &ast::InlineExpression<'source>) -> FluentValue<'source> {
    let (prefix, id, attribute) = match expr {
        ast::InlineExpression::MessageReference { id, attribute } => (None, id.name, attribute),
        ast::InlineExpression::TermReference { id, attribute, .. } => {
            (Some("-"), id.name, attribute)
        }
        _ => unimplemented!(),
    };

    if let Some(attribute) = attribute {
        if let Some(prefix) = prefix {
            FluentValue::None(Some(format!("{}{}.{}", prefix, id, attribute.name).into()))
        } else {
            FluentValue::None(Some(format!("{}.{}", id, attribute.name).into()))
        }
    } else if let Some(prefix) = prefix {
        FluentValue::None(Some(format!("{}{}", prefix, id).into()))
    } else {
        FluentValue::None(Some(id.into()))
    }
}

impl<'source> ResolveValue<'source> for ast::InlineExpression<'source> {
    fn resolve(&self, mut env: &mut Env<'source>) -> FluentValue<'source> {
        match self {
            ast::InlineExpression::StringLiteral { value } => {
                FluentValue::String(unescape_unicode(value))
            }
            ast::InlineExpression::MessageReference { id, attribute } => {
                let msg = env.bundle.entries.get_message(&id.name);

                if let Some(msg) = msg {
                    if let Some(attr) = attribute {
                        maybe_resolve_attribute(env, &msg.attributes, msg.id.name, attr.name)
                            .unwrap_or_else(|| {
                                env.errors.push(ResolverError::None);
                                format_reference_error(&self)
                            })
                    } else {
                        msg.resolve(env)
                    }
                } else {
                    env.errors.push(ResolverError::None);
                    format_reference_error(&self)
                }
            }
            ast::InlineExpression::NumberLiteral { value } => FluentValue::into_number(*value),
            ast::InlineExpression::TermReference {
                id,
                attribute,
                arguments,
            } => {
                let term = env.bundle.entries.get_term(&id.name);

                let (_, resolved_named_args) = get_arguments(env, arguments);

                env.local_args = Some(resolved_named_args);

                let value = if let Some(term) = term {
                    if let Some(attr) = attribute {
                        maybe_resolve_attribute(env, &term.attributes, term.id.name, attr.name)
                            .unwrap_or_else(|| {
                                env.errors.push(ResolverError::None);
                                format_reference_error(&self)
                            })
                    } else {
                        term.resolve(&mut env)
                    }
                } else {
                    env.errors.push(ResolverError::None);
                    format_reference_error(&self)
                };
                env.local_args = None;
                value
            }
            ast::InlineExpression::FunctionReference { id, arguments } => {
                let (resolved_positional_args, resolved_named_args) = get_arguments(env, arguments);

                let func = env.bundle.entries.get_function(id.name);

                if let Some(func) = func {
                    func(resolved_positional_args.as_slice(), &resolved_named_args)
                } else {
                    FluentValue::None(None)
                }
            }
            ast::InlineExpression::VariableReference { id } => {
                if let Some(args) = &env.local_args {
                    match args.get(&id.name) {
                        Some(arg) => arg.clone(),
                        None => FluentValue::None(Some(format!("${}", id.name).into())),
                    }
                } else {
                    match env.args.and_then(|args| args.get(&id.name)) {
                        Some(arg) => arg.clone(),
                        None => FluentValue::None(Some(format!("${}", id.name).into())),
                    }
                }
            }
            ast::InlineExpression::Placeable { expression } => expression.resolve(env),
        }
    }
}

fn get_arguments<'env>(
    env: &mut Env<'env>,
    arguments: &Option<ast::CallArguments<'env>>,
) -> (
    Vec<FluentValue<'env>>,
    HashMap<&'env str, FluentValue<'env>>,
) {
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
