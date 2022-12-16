use std::iter::FromIterator;

use crate::types::FluentValue;

/// Fluent messages can use arguments in order to programmatically add values to a
/// translated string. For instance, in a localized application you may wish to display
/// a user's email count. This could be done with the following message.
///
/// `msg-key = Hello, { $user }. You have { $emailCount } messages.`
///
/// Here `$user` and `$emailCount` are the arguments, which can be filled with values.
///
/// The [`FluentArgs`] struct is the map from the argument name (for example `$user`) to
/// the argument value (for example "John".) The logic to apply these to write these
/// to messages is elsewhere, this struct just stores the value.
///
/// # Example
///
/// ```
/// use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
///
/// let mut args = FluentArgs::new();
/// args.set("user", "John");
/// args.set("emailCount", 5);
///
/// let res = FluentResource::try_new(r#"
///
/// msg-key = Hello, { $user }. You have { $emailCount } messages.
///
/// "#.to_string())
///     .expect("Failed to parse FTL.");
///
/// let mut bundle = FluentBundle::default();
///
/// // For this example, we'll turn on BiDi support.
/// // Please, be careful when doing it, it's a risky move.
/// bundle.set_use_isolating(false);
///
/// bundle.add_resource(res)
///     .expect("Failed to add a resource.");
///
/// let mut err = vec![];
///
/// let msg = bundle.get_message("msg-key")
///     .expect("Failed to retrieve a message.");
/// let value = msg.value()
///     .expect("Failed to retrieve a value.");
///
/// assert_eq!(
///     bundle.format_pattern(value, Some(&args), &mut err),
///     "Hello, John. You have 5 messages."
/// );
/// ```
#[derive(Debug, Default)]
pub struct FluentArgs<'args>(Vec<(&'args str, FluentValue<'args>)>);

impl<'args> FluentArgs<'args> {
    /// Creates a new empty argument map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pre-allocates capacity for arguments.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Gets the [`FluentValue`] at the `key` if it exists.
    pub fn get<'s>(&'s self, key: &str) -> Option<&'s FluentValue<'args>> {
        if let Ok(idx) = self.0.binary_search_by_key(&key, |(k, _)| k) {
            Some(&self.0[idx].1)
        } else {
            None
        }
    }

    /// Sets the key value pair.
    pub fn set<V>(&mut self, key: &'args str, value: V)
    where
        V: Into<FluentValue<'args>>,
    {
        self.set_inner(key, value.into());
    }

    fn set_inner(&mut self, key: &'args str, value: FluentValue<'args>) {
        match self.0.binary_search_by_key(&&key, |(k, _)| k) {
            Ok(idx) => self.0[idx] = (key, value),
            Err(idx) => self.0.insert(idx, (key, value)),
        };
    }

    /// Iterate over a tuple of the key an [`FluentValue`].
    pub fn iter(&self) -> impl Iterator<Item = (&'args str, &FluentValue<'args>)> {
        self.0.iter().map(|(k, v)| (*k, v))
    }
}

impl<'args, V> FromIterator<(&'args str, V)> for FluentArgs<'args>
where
    V: Into<FluentValue<'args>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (&'args str, V)>,
    {
        let iter = iter.into_iter();
        let mut args = if let Some(size) = iter.size_hint().1 {
            FluentArgs::with_capacity(size)
        } else {
            FluentArgs::new()
        };

        for (k, v) in iter {
            args.set(k, v);
        }

        args
    }
}

impl<'args> IntoIterator for FluentArgs<'args> {
    type Item = (&'args str, FluentValue<'args>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;

    #[test]
    fn replace_existing_arguments() {
        let mut args = FluentArgs::new();

        args.set("name", "John");
        args.set("emailCount", 5);
        assert_eq!(args.0.len(), 2);
        assert_eq!(
            args.get("name"),
            Some(&FluentValue::String(Cow::Borrowed("John")))
        );
        assert_eq!(args.get("emailCount"), Some(&FluentValue::try_number("5")));

        args.set("name", "Jane");
        args.set("emailCount", 7);
        assert_eq!(args.0.len(), 2);
        assert_eq!(
            args.get("name"),
            Some(&FluentValue::String(Cow::Borrowed("Jane")))
        );
        assert_eq!(args.get("emailCount"), Some(&FluentValue::try_number("7")));
    }
}
