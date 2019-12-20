use intl_memoizer::{IntlMemoizer, Memoizable};
use unic_langid::LanguageIdentifier;
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

fn main() {
    let mut memoizer = IntlMemoizer::new();

    let lang: LanguageIdentifier = "en".parse().unwrap();
    let lang_memoizer = memoizer.get_for_lang(lang.clone());
    let mut lang_memoizer_borrow = lang_memoizer.borrow_mut();

    let pr = lang_memoizer_borrow.get::<PluralRules>((PluralRuleType::CARDINAL,));

    assert_eq!(pr.0.select(5), Ok(PluralCategory::OTHER));
}
