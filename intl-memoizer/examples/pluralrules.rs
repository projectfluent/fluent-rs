use icu_locid::LanguageIdentifier;
use icu_plurals::{PluralCategory, PluralRuleType, PluralRules as IntlPluralRules};
use intl_memoizer::{IntlMemoizer, Memoizable};

struct PluralRules(pub IntlPluralRules);

impl PluralRules {
    pub fn new(lang: LanguageIdentifier, pr_type: PluralRuleType) -> Result<Self, &'static str> {
        let locale = lang.into();
        let inner = match pr_type {
            PluralRuleType::Cardinal => IntlPluralRules::try_new_cardinal(&locale),
            PluralRuleType::Ordinal => IntlPluralRules::try_new_ordinal(&locale),
            _ => todo!(),
        };
        Ok(Self(inner.unwrap()))
    }
}

impl Memoizable for PluralRules {
    type Args = (PluralRuleType,);
    type Error = &'static str;
    fn construct(lang: LanguageIdentifier, args: Self::Args) -> Result<Self, Self::Error> {
        Self::new(lang, args.0)
    }
}

fn main() {
    let mut memoizer = IntlMemoizer::default();

    let lang: LanguageIdentifier = "en".parse().unwrap();
    let lang_memoizer = memoizer.get_for_lang(lang);
    let result = lang_memoizer
        .with_try_get::<PluralRules, _, _>((PluralRuleType::Cardinal,), |pr| pr.0.category_for(5))
        .unwrap();

    assert_eq!(result, PluralCategory::Other);
}
