extern crate intl_pluralrules;
use self::intl_pluralrules::{IntlPluralRules, PluralCategory, PluralRuleType};

#[test]
fn plural_rules() {
    let pr = IntlPluralRules::create("pl", PluralRuleType::CARDINAL).unwrap();

    assert_eq!(pr.select(0.0), Ok(PluralCategory::MANY));
    assert_eq!(pr.select(1.0), Ok(PluralCategory::ONE));
    assert_eq!(pr.select(2.0), Ok(PluralCategory::FEW));
    assert_eq!(pr.select(5.0), Ok(PluralCategory::MANY));

    assert_eq!(pr.select(1.5), Ok(PluralCategory::OTHER));
}
