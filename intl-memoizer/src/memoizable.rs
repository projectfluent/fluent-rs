use icu_locid::LanguageIdentifier;
use std::hash::Hash;

pub trait Memoizable {
    type Args: 'static + Eq + Hash + Clone;
    type Provider;

    type Error: std::fmt::Debug;

    fn construct(
        lang: LanguageIdentifier,
        args: Self::Args,
        provider: Option<&Self::Provider>,
    ) -> Result<Self, Self::Error>
    where
        Self: std::marker::Sized;
}
