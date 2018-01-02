extern crate fluent;

use self::fluent::intl::PluralRules;

#[test]
fn plural_rules() {
    let pr = PluralRules::new(&["pl"]);

    assert_eq!(pr.select(0.0), "many");
    assert_eq!(pr.select(1.0), "one");
    assert_eq!(pr.select(2.0), "few");
    assert_eq!(pr.select(5.0), "many");

    assert_eq!(pr.select(1.5), "other");
}

#[test]
fn plural_rules_default() {
    let pr = PluralRules::new(&["xx"]);

    assert_eq!(pr.locale, "en");
}
