use crate::bundle::FluentBundle;
use crate::memoizer::MemoizerKind;
use crate::types::FluentValue;
use crate::{FluentArgs, FluentError, FluentResource};
use fluent_syntax::ast;
use std::borrow::Borrow;

use super::{ResolveContext, ResolverError, WriteOrResolve, WriteOrResolveContext};

/// State for a single `WriteOrResolve::write_or_resolve` call.
pub struct Scope<'bundle, 'other, R, M> {
    /// The current `FluentBundle` instance.
    pub bundle: &'bundle FluentBundle<R, M>,
    /// The current arguments passed by the developer.
    pub(super) args: Option<&'other FluentArgs<'other>>,
    /// Local args
    pub(super) local_args: Option<FluentArgs<'bundle>>,
    /// The running count of resolved placeables. Used to detect the Billion
    /// Laughs and Quadratic Blowup attacks.
    pub(super) placeables: u8,
    /// Tracks hashes to prevent infinite recursion.
    travelled: smallvec::SmallVec<[&'bundle ast::Pattern<&'bundle str>; 2]>,
    /// Track errors accumulated during resolving.
    pub errors: Option<&'other mut Vec<FluentError>>,
    /// Makes the resolver bail.
    pub dirty: bool,
}

impl<'bundle, 'other, R, M> Scope<'bundle, 'other, R, M> {
    pub fn new(
        bundle: &'bundle FluentBundle<R, M>,
        args: Option<&'other FluentArgs>,
        errors: Option<&'other mut Vec<FluentError>>,
    ) -> Self {
        Scope {
            bundle,
            args,
            local_args: None,
            placeables: 0,
            travelled: Default::default(),
            errors,
            dirty: false,
        }
    }

    pub fn add_error(&mut self, error: ResolverError) {
        if let Some(errors) = self.errors.as_mut() {
            errors.push(error.into());
        }
    }

    /// This method allows us to lazily add Pattern on the stack, only if the
    /// `Pattern::resolve` has been called on an empty stack.
    ///
    /// This is the case when pattern is called from Bundle and it allows us to fast-path
    /// simple resolutions, and only use the stack for placeables.
    pub fn maybe_track<T>(
        &mut self,
        context: &mut T,
        pattern: &'bundle ast::Pattern<&'bundle str>,
        exp: &'bundle ast::Expression<&'bundle str>,
    ) -> T::Result
    where
        R: Borrow<FluentResource>,
        M: MemoizerKind,
        T: WriteOrResolveContext<'bundle>,
    {
        if self.travelled.is_empty() {
            self.travelled.push(pattern);
        }
        let res = exp.write_or_resolve(self, context);
        if self.dirty {
            context.error(exp, true)
        } else {
            res
        }
    }

    pub fn track<T>(
        &mut self,
        context: &mut T,
        pattern: &'bundle ast::Pattern<&'bundle str>,
        exp: &'bundle ast::InlineExpression<&'bundle str>,
    ) -> T::Result
    where
        R: Borrow<FluentResource>,
        M: MemoizerKind,
        T: WriteOrResolveContext<'bundle>,
    {
        if self.travelled.contains(&pattern) {
            self.add_error(ResolverError::Cyclic);
            context.error(exp, true)
        } else {
            self.travelled.push(pattern);
            let result = context.resolve_pattern(self, pattern);
            self.travelled.pop();
            result
        }
    }

    pub fn get_arguments(
        &mut self,
        arguments: Option<&'bundle ast::CallArguments<&'bundle str>>,
    ) -> (Vec<FluentValue<'bundle>>, FluentArgs<'bundle>)
    where
        R: Borrow<FluentResource>,
        M: MemoizerKind,
    {
        if let Some(ast::CallArguments { positional, named }) = arguments {
            let positional = positional
                .iter()
                .map(|expr| expr.write_or_resolve(self, &mut ResolveContext))
                .collect();

            let named = named
                .iter()
                .map(|arg| {
                    (
                        arg.name.name,
                        arg.value.write_or_resolve(self, &mut ResolveContext),
                    )
                })
                .collect();

            (positional, named)
        } else {
            (Vec::new(), FluentArgs::new())
        }
    }
}
