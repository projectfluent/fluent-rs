use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use typemap::{Key, TypeMap};
use unic_langid::LanguageIdentifier;

pub trait Memoizable {
    type Args: 'static + Eq + Hash + Clone;
    type Error;
    fn construct(lang: LanguageIdentifier, args: Self::Args) -> Result<Self, Self::Error>
    where
        Self: std::marker::Sized;
}

struct MemoizeKey<T>(T);

impl<T: Memoizable + 'static> Key for MemoizeKey<T> {
    type Value = HashMap<T::Args, T>;
}

pub struct IntlLangMemoizer {
    lang: LanguageIdentifier,
    map: TypeMap,
}

impl IntlLangMemoizer {
    pub fn new(lang: LanguageIdentifier) -> Self {
        Self {
            lang,
            map: TypeMap::new(),
        }
    }

    pub fn get<T: Memoizable + 'static>(&mut self, args: T::Args) -> Result<&T, T::Error>
    where
        T::Args: Eq,
    {
        let lang = self.lang.clone();
        let cache = self
            .map
            .entry::<MemoizeKey<T>>()
            .or_insert_with(|| HashMap::new());

        let e = match cache.entry(args.clone()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let val = T::construct(lang, args.clone())?;
                entry.insert(val)
            }
        };
        Ok(e)
    }
}

pub struct IntlMemoizer {
    map: HashMap<LanguageIdentifier, Weak<RefCell<IntlLangMemoizer>>>,
}

impl IntlMemoizer {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get_for_lang(&mut self, lang: LanguageIdentifier) -> Rc<RefCell<IntlLangMemoizer>> {
        match self.map.entry(lang.clone()) {
            Entry::Vacant(empty) => {
                let entry = Rc::new(RefCell::new(IntlLangMemoizer::new(lang.clone())));
                empty.insert(Rc::downgrade(&entry));
                entry
            }
            Entry::Occupied(mut entry) => {
                if let Some(entry) = entry.get().upgrade() {
                    entry
                } else {
                    let e = Rc::new(RefCell::new(IntlLangMemoizer::new(lang.clone())));
                    entry.insert(Rc::downgrade(&e));
                    e
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use intl_pluralrules::{PluralCategory, PluralRuleType, PluralRules as IntlPluralRules};

    struct PluralRules(pub IntlPluralRules);

    impl PluralRules {
        pub fn new(
            lang: LanguageIdentifier,
            pr_type: PluralRuleType,
        ) -> Result<Self, &'static str> {
            Ok(Self(IntlPluralRules::create(lang, pr_type)?))
        }
    }

    impl Memoizable for PluralRules {
        type Args = (PluralRuleType,);
        type Error = &'static str;
        fn construct(lang: LanguageIdentifier, args: Self::Args) -> Result<Self, Self::Error> {
            Self::new(lang, args.0)
        }
    }

    #[test]
    fn it_works() {
        let lang: LanguageIdentifier = "en".parse().unwrap();

        let mut memoizer = IntlMemoizer::new();
        let en_memoizer = memoizer.get_for_lang(lang.clone());
        let mut en_memoizer_borrow = en_memoizer.borrow_mut();

        let cb = en_memoizer_borrow
            .get::<PluralRules>((PluralRuleType::CARDINAL,))
            .unwrap();
        assert_eq!(cb.0.select(5), Ok(PluralCategory::OTHER));
    }
}
