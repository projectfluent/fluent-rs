use crate::bundle::FluentBundle;
use crate::memoizer::MemoizerKind;
use crate::resolver::{ResolveValue, ResolverError, WriteValue};
use crate::types::FluentValue;
use crate::{FluentArgs, FluentError, FluentResource};
use fluent_syntax::ast;
use std::borrow::Borrow;
use std::fmt;

/// State for a single `ResolveValue::to_value` call.
pub struct Scope<'bundle, 'ast, 'args, 'errors, R, M> {
    /// The current `FluentBundle` instance.
    pub bundle: &'bundle FluentBundle<R, M>,
    /// The current arguments passed by the developer.
    pub(super) args: Option<&'args FluentArgs<'args>>,
    /// Local args
    pub(super) local_args: Option<FluentArgs<'bundle>>,
    /// The running count of resolved placeables. Used to detect the Billion
    /// Laughs and Quadratic Blowup attacks.
    pub(super) placeables: u8,
    /// Tracks hashes to prevent infinite recursion.
    traveled: smallvec::SmallVec<[&'ast ast::Pattern<&'bundle str>; 2]>,
    /// Track errors accumulated during resolving.
    pub errors: Option<&'errors mut Vec<FluentError>>,
    /// Makes the resolver bail.
    pub dirty: bool,
}

impl<'bundle, 'ast, 'args, 'errors, R, M> Scope<'bundle, 'ast, 'args, 'errors, R, M> {
    pub fn new(
        bundle: &'bundle FluentBundle<R, M>,
        args: Option<&'args FluentArgs>,
        errors: Option<&'errors mut Vec<FluentError>>,
    ) -> Self {
        Scope {
            bundle,
            args,
            local_args: None,
            placeables: 0,
            traveled: Default::default(),
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
    pub fn maybe_track<W>(
        &mut self,
        w: &mut W,
        pattern: &'ast ast::Pattern<&'bundle str>,
        exp: &'ast ast::Expression<&'bundle str>,
    ) -> fmt::Result
    where
        R: Borrow<FluentResource>,
        W: fmt::Write,
        M: MemoizerKind,
    {
        if self.traveled.is_empty() {
            self.traveled.push(pattern);
        }
        exp.write(w, self)?;
        if self.dirty {
            w.write_char('{')?;
            exp.write_error(w)?;
            w.write_char('}')
        } else {
            Ok(())
        }
    }

    pub fn track<W>(
        &mut self,
        w: &mut W,
        pattern: &'ast ast::Pattern<&'bundle str>,
        exp: &'ast ast::InlineExpression<&'bundle str>,
    ) -> fmt::Result
    where
        R: Borrow<FluentResource>,
        W: fmt::Write,
        M: MemoizerKind,
    {
        if self.traveled.contains(&pattern) {
            self.add_error(ResolverError::Cyclic);
            w.write_char('{')?;
            exp.write_error(w)?;
            w.write_char('}')
        } else {
            self.traveled.push(pattern);
            let result = pattern.write(w, self);
            self.traveled.pop();
            result
        }
    }

    pub fn write_ref_error<W>(
        &mut self,
        w: &mut W,
        exp: &ast::InlineExpression<&str>,
    ) -> fmt::Result
    where
        W: fmt::Write,
    {
        self.add_error(exp.into());
        w.write_char('{')?;
        exp.write_error(w)?;
        w.write_char('}')
    }

    pub fn get_arguments(
        &mut self,
        arguments: Option<&'ast ast::CallArguments<&'bundle str>>,
    ) -> (Vec<FluentValue<'bundle>>, FluentArgs<'bundle>)
    where
        R: Borrow<FluentResource>,
        M: MemoizerKind,
    {
        if let Some(ast::CallArguments { positional, named }) = arguments {
            let positional = positional.iter().map(|expr| expr.resolve(self)).collect();

            let named = named
                .iter()
                .map(|arg| (arg.name.name, arg.value.resolve(self)))
                .collect();

            (positional, named)
        } else {
            (Vec::new(), FluentArgs::new())
        }
    }
}
