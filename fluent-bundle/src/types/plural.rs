use icu_locid::LanguageIdentifier;
use icu_plurals::{PluralRuleType, PluralRules as IntlPluralRules};
use intl_memoizer::Memoizable;

pub struct PluralRules(pub IntlPluralRules);

impl Memoizable for PluralRules {
    type Args = (PluralRuleType,);
    type Error = &'static str;
    fn construct(lang: LanguageIdentifier, args: Self::Args) -> Result<Self, Self::Error> {
        let inner = match args.0 {
            PluralRuleType::Cardinal => IntlPluralRules::try_new_cardinal(&lang.into()),
            PluralRuleType::Ordinal => IntlPluralRules::try_new_ordinal(&lang.into()),
            _ => todo!(),
        };
        Ok(Self(inner.unwrap()))
    }
}
