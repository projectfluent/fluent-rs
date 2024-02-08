// use std::collections::HashMap;
use crate::memoizable::Memoizable;
use hashbrown::HashMap;
use icu_locid::LanguageIdentifier;
use std::cell::RefCell;

pub struct IntlLangMemoizer<'dp, DP = ()> {
    lang: LanguageIdentifier,
    provider: Option<&'dp DP>,
    map: RefCell<type_map::TypeMap>,
}

impl<'dp, DP> IntlLangMemoizer<'dp, DP> {
    pub fn new(lang: LanguageIdentifier, provider: Option<&'dp DP>) -> Self {
        Self {
            lang,
            provider,
            map: Default::default(),
        }
    }

    pub fn with_try_get<I, R, U>(
        &self,
        construct_args: &I::Args,
        callback: U,
    ) -> Result<R, I::Error>
    where
        Self: Sized,
        I: Memoizable<Provider = DP> + 'static,
        U: FnOnce(&I) -> R,
    {
        let mut map = self.map.borrow_mut();

        let cache = map.entry().or_insert_with(HashMap::<I::Args, I>::new);

        let (_, e) = cache
            .raw_entry_mut()
            .from_key(construct_args)
            .or_insert_with(|| {
                (
                    construct_args.clone(),
                    I::construct(self.lang.clone(), construct_args.clone(), self.provider)
                        .expect("FOO"),
                )
            });
        Ok(callback(e))
    }
}
