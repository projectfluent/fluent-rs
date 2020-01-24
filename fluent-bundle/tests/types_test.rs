use fluent_bundle::resolve::Scope;
use fluent_bundle::FluentBundle;
use fluent_bundle::FluentResource;
use fluent_bundle::FluentValue;
use unic_langid::langid;

#[test]
fn fluent_value_number() {
    let value = FluentValue::try_number("invalid");
    assert_eq!(value, FluentValue::String("invalid".into()));
}

#[test]
fn fluent_value_matches() {
    // We'll use `ars` locale since it happens to have all
    // plural rules categories.
    let langid_ars = langid!("ars");
    let bundle: FluentBundle<FluentResource> = FluentBundle::new(&[langid_ars]);
    let scope = Scope::new(&bundle, None);

    let string_val = FluentValue::String("string1".into());
    let string_val_copy = FluentValue::String("string1".into());
    let string_val2 = FluentValue::String("23.5".into());

    let number_val = FluentValue::try_number(-23.5);
    let number_val_copy = FluentValue::try_number(-23.5);
    let number_val2 = FluentValue::try_number(23.5);

    assert_eq!(string_val.matches(&string_val_copy, &scope), true);
    assert_eq!(string_val.matches(&string_val2, &scope), false);

    assert_eq!(number_val.matches(&number_val_copy, &scope), true);
    assert_eq!(number_val.matches(&number_val2, &scope), false);

    assert_eq!(string_val2.matches(&number_val2, &scope), false);

    assert_eq!(string_val2.matches(&number_val2, &scope), false);

    let string_cat_zero = FluentValue::String("zero".into());
    let string_cat_one = FluentValue::String("one".into());
    let string_cat_two = FluentValue::String("two".into());
    let string_cat_few = FluentValue::String("few".into());
    let string_cat_many = FluentValue::String("many".into());
    let string_cat_other = FluentValue::String("other".into());

    let number_cat_zero = FluentValue::try_number(0);
    let number_cat_one = FluentValue::try_number(1);
    let number_cat_two = FluentValue::try_number(2);
    let number_cat_few = FluentValue::try_number(3);
    let number_cat_many = FluentValue::try_number(11);
    let number_cat_other = FluentValue::try_number(101);

    assert_eq!(string_cat_zero.matches(&number_cat_zero, &scope), true);
    assert_eq!(string_cat_one.matches(&number_cat_one, &scope), true);
    assert_eq!(string_cat_two.matches(&number_cat_two, &scope), true);
    assert_eq!(string_cat_few.matches(&number_cat_few, &scope), true);
    assert_eq!(string_cat_many.matches(&number_cat_many, &scope), true);
    assert_eq!(string_cat_other.matches(&number_cat_other, &scope), true);
    assert_eq!(string_cat_other.matches(&number_cat_one, &scope), false);

    assert_eq!(string_val2.matches(&number_cat_one, &scope), false);
}

#[test]
fn fluent_value_from() {
    let value_str = FluentValue::from("my str");
    let value_string = FluentValue::from(String::from("my string"));
    let value_f64 = FluentValue::from(23.5 as f64);
    let value_isize = FluentValue::from(-23 as isize);

    assert_eq!(value_str, FluentValue::String("my str".into()));
    assert_eq!(value_string, FluentValue::String("my string".into()));

    assert_eq!(value_f64, FluentValue::try_number(23.5));
    assert_eq!(value_isize, FluentValue::try_number(-23));
}
