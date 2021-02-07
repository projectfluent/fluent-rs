use std::borrow::Cow;
use std::iter::FromIterator;

use crate::types::FluentValue;

/// A map of arguments passed from the code to
/// the localization to be used for message
/// formatting.
#[derive(Debug, Default)]
pub struct FluentArgs<'args>(Vec<(Cow<'args, str>, FluentValue<'args>)>);

impl<'args> FluentArgs<'args> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn get(&self, key: &str) -> Option<&FluentValue<'args>> {
        self.0.iter().find(|(k, _)| key == *k).map(|(_, v)| v)
    }

    pub fn add<K, V>(&mut self, key: K, value: V)
    where
        K: Into<Cow<'args, str>>,
        V: Into<FluentValue<'args>>,
    {
        self.0.push((key.into(), value.into()));
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &FluentValue)> {
        self.0.iter().map(|(k, v)| (k.as_ref(), v))
    }
}

impl<'args, K, V> FromIterator<(K, V)> for FluentArgs<'args>
where
    K: Into<Cow<'args, str>>,
    V: Into<FluentValue<'args>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut c = FluentArgs::new();

        for (k, v) in iter {
            c.add(k, v);
        }

        c
    }
}

impl<'args> IntoIterator for FluentArgs<'args> {
    type Item = (Cow<'args, str>, FluentValue<'args>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
