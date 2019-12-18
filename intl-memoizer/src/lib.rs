use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use typemap::{Key, TypeMap};
use unic_langid::LanguageIdentifier;

pub trait Memoizable {
    type Args: 'static + Eq + Hash + Clone;
    fn construct(args: Self::Args) -> Self;
}

struct MemoizeKey<T>(T);

impl<T: Memoizable + 'static> Key for MemoizeKey<T> {
    type Value = HashMap<T::Args, Rc<T>>;
}

pub struct IntlMemoizer {
    map: TypeMap,
}

impl IntlMemoizer {
    pub fn new() -> Self {
        Self {
            map: TypeMap::new(),
        }
    }

    pub fn get<T: Memoizable + 'static>(&mut self, args: T::Args) -> Rc<T> {
        let cache = self
            .map
            .entry::<MemoizeKey<T>>()
            .or_insert_with(|| HashMap::new());
        // not using entry to avoid unnecessary cloning
        if let Some(val) = cache.get(&args) {
            val.clone()
        } else {
            let val = Rc::new(T::construct(args.clone()));
            cache.insert(args.clone(), val.clone());
            val
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
    enum PluralRulesType {
        Cardinal,
        Ordinal,
    }

    struct PluralRules {
        lang: LanguageIdentifier,
        pr_type: PluralRulesType,
    }

    impl PluralRules {
        pub fn new(lang: LanguageIdentifier, pr_type: PluralRulesType) -> Self {
            Self { lang, pr_type }
        }

        pub fn select(&self) -> String {
            format!("Selected for {} and {:#?}", self.lang, self.pr_type)
        }
    }

    impl Memoizable for PluralRules {
        type Args = (LanguageIdentifier, PluralRulesType);
        fn construct(args: Self::Args) -> Self {
            Self::new(args.0, args.1)
        }
    }

    struct DateTimeFormat {}
    impl DateTimeFormat {
        pub fn new() -> Self {
            Self {}
        }
        pub fn format() {}
    }

    #[test]
    fn it_works() {
        let mut memoizer = IntlMemoizer::new();

        let cb = memoizer.get::<PluralRules>(("en-US".parse().unwrap(), PluralRulesType::Cardinal));
        assert_eq!(cb.select(), "Selected for en-US and Cardinal");
    }
}
