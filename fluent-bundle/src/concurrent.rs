use intl_memoizer::{concurrent::IntlLangMemoizer, Memoizable};
use unic_langid::LanguageIdentifier;

use crate::bundle::FluentBundle;
use crate::memoizer::MemoizerKind;
use crate::types::FluentType;

impl<R> FluentBundle<R, IntlLangMemoizer> {
    pub fn new_concurrent() {}
}

impl MemoizerKind for IntlLangMemoizer {
    fn new(lang: LanguageIdentifier) -> Self
    where
        Self: Sized,
    {
        Self::new(lang)
    }

    fn with_try_get_threadsafe<I, R, U>(&self, args: I::Args, cb: U) -> Result<R, I::Error>
    where
        Self: Sized,
        I: Memoizable + Send + Sync + 'static,
        I::Args: Send + Sync + 'static,
        U: FnOnce(&I) -> R,
    {
        self.with_try_get(args, cb)
    }

    fn stringify_value(&self, value: &dyn FluentType) -> std::borrow::Cow<'static, str> {
        value.as_string_threadsafe(self)
    }
}
