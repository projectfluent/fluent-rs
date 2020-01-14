use intl_memoizer::{IntlMemoizer, Memoizable};
use intl_pluralrules::{PluralCategory, PluralRuleType, PluralRules as IntlPluralRules};
use unic_langid::LanguageIdentifier;

struct PluralRules(pub IntlPluralRules);

impl PluralRules {
    pub fn new(lang: LanguageIdentifier, pr_type: PluralRuleType) -> Result<Self, &'static str> {
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

fn main() {
    let mut memoizer = IntlMemoizer::default();

    let lang: LanguageIdentifier = "en".parse().unwrap();
    let lang_memoizer = memoizer.get_for_lang(lang.clone());
    let mut lang_memoizer_borrow = lang_memoizer.borrow_mut();

    let pr = lang_memoizer_borrow
        .try_get::<PluralRules>((PluralRuleType::CARDINAL,))
        .unwrap();

    assert_eq!(pr.0.select(5), Ok(PluralCategory::OTHER));
}
