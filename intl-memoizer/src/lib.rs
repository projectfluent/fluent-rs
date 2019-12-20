use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use typemap::{Key, TypeMap};
use unic_langid::LanguageIdentifier;

pub trait Memoizable {
    type Args: 'static + Eq + Hash + Clone;
    fn construct(lang: LanguageIdentifier, args: Self::Args) -> Self;
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

    pub fn get<T: Memoizable + 'static>(&mut self, args: T::Args) -> &T
    where
        T::Args: Eq,
    {
        let lang = self.lang.clone();
        let cache = self
            .map
            .entry::<MemoizeKey<T>>()
            .or_insert_with(|| HashMap::new());
        // not using entry to avoid unnecessary cloning
        let val = cache
            .entry(args.clone())
            .or_insert_with(|| T::construct(lang, args.clone()));
        val
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
    use intl_pluralrules::{PluralRules as IntlPluralRules, PluralRuleType, PluralCategory};

    struct PluralRules(pub IntlPluralRules);

    impl PluralRules {
        pub fn new(lang: LanguageIdentifier, pr_type: PluralRuleType) -> Self {
            Self(IntlPluralRules::create(lang, pr_type).unwrap())
        }
    }

    impl Memoizable for PluralRules {
        type Args = (PluralRuleType,);
        fn construct(lang: LanguageIdentifier, args: Self::Args) -> Self {
            Self::new(lang, args.0)
        }
    }

    #[test]
    fn it_works() {
        let lang: LanguageIdentifier = "en".parse().unwrap();

        let mut memoizer = IntlMemoizer::new();
        let en_memoizer = memoizer.get_for_lang(lang.clone());
        let mut en_memoizer_borrow = en_memoizer.borrow_mut();

        let cb = en_memoizer_borrow.get::<PluralRules>((PluralRuleType::CARDINAL,));
        assert_eq!(cb.0.select(5), Ok(PluralCategory::OTHER));
    }
}
