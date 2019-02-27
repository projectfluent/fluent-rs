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

#[derive(Debug, PartialEq)]
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
    pub args: Option<&'env HashMap<&'env str, FluentValue>>,
    /// Tracks hashes to prevent infinite recursion.
    pub travelled: RefCell<Vec<u64>>,
}

impl<'env> Env<'env> {
    pub fn new(
        bundle: &'env FluentBundle,
        args: Option<&'env HashMap<&'env str, FluentValue>>,
    ) -> Self {
        Env {
            bundle,
            args,
            travelled: RefCell::new(Vec::new()),
        }
    }

    fn track<F>(&self, identifier: &str, action: F) -> Result<FluentValue, ResolverError>
    where
        F: FnMut() -> Result<FluentValue, ResolverError>,
    {
        let mut hasher = DefaultHasher::new();
        identifier.hash(&mut hasher);
        let hash = hasher.finish();

        if self.travelled.borrow().contains(&hash) {
            Err(ResolverError::Cyclic)
        } else {
            self.travelled.borrow_mut().push(hash);
            self.scope(action)
        }
    }

    fn scope<T, F: FnMut() -> T>(&self, mut action: F) -> T {
        let level = self.travelled.borrow().len();
        let output = action();
        self.travelled.borrow_mut().truncate(level);
        output
    }
}

/// Converts an AST node to a `FluentValue`.
pub trait ResolveValue {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError>;
}

impl<'source> ResolveValue for ast::Message<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        env.track(&self.id.name, || {
            self.value
                .as_ref()
                .ok_or(ResolverError::None)?
                .to_value(env)
        })
    }
}

impl<'source> ResolveValue for ast::Term<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        env.track(&self.id.name, || self.value.to_value(env))
    }
}

impl<'source> ResolveValue for ast::Attribute<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        env.track(&self.id.name, || self.value.to_value(env))
    }
}

impl<'source> ResolveValue for ast::Pattern<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        let mut string = String::with_capacity(128);
        for elem in &self.elements {
            let result: Result<String, ()> = env.scope(|| match elem.to_value(env) {
                Err(ResolverError::Cyclic) => Err(()),
                Err(_) => Ok("___".into()),
                Ok(elem) => Ok(elem.format(env.bundle)),
            });

            match result {
                Err(()) => return Ok("___".into()),
                Ok(value) => {
                    string.push_str(&value);
                }
            }
        }
        string.shrink_to_fit();
        Ok(FluentValue::from(string))
    }
}

impl<'source> ResolveValue for ast::PatternElement<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        match self {
            ast::PatternElement::TextElement(s) => Ok(FluentValue::from(*s)),
            ast::PatternElement::Placeable(p) => p.to_value(env),
        }
    }
}

impl<'source> ResolveValue for ast::VariantKey<'source> {
    fn to_value(&self, _env: &Env) -> Result<FluentValue, ResolverError> {
        match self {
            ast::VariantKey::Identifier { name } => Ok(FluentValue::from(*name)),
            ast::VariantKey::NumberLiteral { value } => {
                FluentValue::into_number(value).map_err(|_| ResolverError::Value)
            }
        }
    }
}

impl<'source> ResolveValue for ast::Expression<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        match self {
            ast::Expression::InlineExpression(exp) => exp.to_value(env),
            ast::Expression::SelectExpression { selector, variants } => {
                if let Ok(ref selector) = selector.to_value(env) {
                    for variant in variants {
                        match variant.key {
                            ast::VariantKey::Identifier { name } => {
                                let key = FluentValue::from(name);
                                if key.matches(env.bundle, selector) {
                                    return variant.value.to_value(env);
                                }
                            }
                            ast::VariantKey::NumberLiteral { value } => {
                                if let Ok(key) = FluentValue::into_number(value) {
                                    if key.matches(env.bundle, selector) {
                                        return variant.value.to_value(env);
                                    }
                                } else {
                                    return Err(ResolverError::Value);
                                }
                            }
                        }
                    }
                }

                select_default(variants)
                    .ok_or(ResolverError::None)?
                    .value
                    .to_value(env)
            }
        }
    }
}

impl<'source> ResolveValue for ast::InlineExpression<'source> {
    fn to_value(&self, env: &Env) -> Result<FluentValue, ResolverError> {
        match self {
            ast::InlineExpression::StringLiteral { raw } => {
                Ok(FluentValue::from(unescape_unicode(raw).into_owned()))
            }
            ast::InlineExpression::NumberLiteral { value } => {
                FluentValue::into_number(*value).map_err(|_| ResolverError::None)
            }
            ast::InlineExpression::FunctionReference { id, arguments } => {
                let (resolved_positional_args, resolved_named_args) = get_arguments(env, arguments);

                let func = env.bundle.entries.get_function(id.name);

                func.ok_or(ResolverError::None).and_then(|func| {
                    func(resolved_positional_args.as_slice(), &resolved_named_args)
                        .ok_or(ResolverError::None)
                })
            }
            ast::InlineExpression::MessageReference { id, attribute } => {
                let msg = env
                    .bundle
                    .entries
                    .get_message(&id.name)
                    .ok_or(ResolverError::None)?;
                if let Some(attribute) = attribute {
                    for attr in msg.attributes.iter() {
                        if attr.id.name == attribute.name {
                            return attr.to_value(env);
                        }
                    }
                    Err(ResolverError::None)
                } else {
                    msg.to_value(env)
                }
            }
            ast::InlineExpression::TermReference {
                id,
                attribute,
                arguments,
            } => {
                let term = env
                    .bundle
                    .entries
                    .get_term(&id.name)
                    .ok_or(ResolverError::None)?;

                let (.., resolved_named_args) = get_arguments(env, arguments);
                let env = Env::new(env.bundle, Some(&resolved_named_args));

                if let Some(attribute) = attribute {
                    for attr in term.attributes.iter() {
                        if attr.id.name == attribute.name {
                            return attr.to_value(&env);
                        }
                    }
                    Err(ResolverError::None)
                } else {
                    term.to_value(&env)
                }
            }
            ast::InlineExpression::VariableReference { id } => env
                .args
                .and_then(|args| args.get(&id.name))
                .cloned()
                .ok_or(ResolverError::None),
            ast::InlineExpression::Placeable { ref expression } => {
                let exp = expression.as_ref();
                exp.to_value(env)
            }
        }
    }
}

fn select_default<'source>(
    variants: &'source [ast::Variant<'source>],
) -> Option<&ast::Variant<'source>> {
    for variant in variants {
        if variant.default {
            return Some(variant);
        }
    }

    None
}

fn get_arguments<'env>(
    env: &Env,
    arguments: &'env Option<ast::CallArguments>,
) -> (Vec<Option<FluentValue>>, HashMap<&'env str, FluentValue>) {
    let mut resolved_positional_args = Vec::new();
    let mut resolved_named_args = HashMap::new();

    if let Some(ast::CallArguments { named, positional }) = arguments {
        for expression in positional {
            resolved_positional_args.push(expression.to_value(env).ok());
        }

        for arg in named {
            if let Ok(arg_value) = arg.value.to_value(env) {
                resolved_named_args.insert(arg.name.name, arg_value);
            }
        }
    }

    (resolved_positional_args, resolved_named_args)
}
