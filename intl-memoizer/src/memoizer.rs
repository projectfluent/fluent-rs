use crate::IntlLangMemoizer;
use icu_locid::LanguageIdentifier;
use std::collections::HashMap;
use std::rc::Rc;

pub struct IntlMemoizer<'dp, DP> {
    provider: Option<&'dp DP>,
    map: HashMap<LanguageIdentifier, Rc<IntlLangMemoizer<'dp, DP>>>,
}

impl<'dp, DP> IntlMemoizer<'dp, DP> {
    pub fn new(provider: Option<&'dp DP>) -> Self {
        Self {
            provider,
            map: HashMap::default(),
        }
    }

    pub fn get_for_lang(&mut self, lang: LanguageIdentifier) -> Rc<IntlLangMemoizer<'dp, DP>> {
        if let Some(memoizer) = self.map.get(&lang) {
            memoizer.clone()
        } else {
            let memoizer = Rc::new(IntlLangMemoizer::new(lang.clone(), self.provider));
            self.map.insert(lang, memoizer.clone());
            memoizer
        }
    }
}
