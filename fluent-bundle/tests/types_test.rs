use fluent_bundle::resolver::Scope;
use fluent_bundle::types::{
    FluentNumber, FluentNumberCurrencyDisplayStyle, FluentNumberOptions, FluentNumberStyle,
    FluentNumberUseGrouping,
};
use fluent_bundle::FluentArgs;
use fluent_bundle::FluentBundle;
use fluent_bundle::FluentResource;
use fluent_bundle::FluentValue;
use intl_pluralrules::operands::PluralOperands;
use unic_langid::langid;

#[test]
fn fluent_value_try_number() {
    let value = FluentValue::try_number("invalid");
    assert_eq!(value, "invalid".into());
}

#[test]
fn fluent_value_matches() {
    // We'll use `ars` locale since it happens to have all
    // plural rules categories.
    let langid_ars = langid!("ars");
    let bundle: FluentBundle<FluentResource> = FluentBundle::new(vec![langid_ars]);
    let scope = Scope::new(&bundle, None, None);

    let string_val = FluentValue::from("string1");
    let string_val_copy = FluentValue::from("string1");
    let string_val2 = FluentValue::from("23.5");

    let number_val = FluentValue::from(-23.5);
    let number_val_copy = FluentValue::from(-23.5);
    let number_val2 = FluentValue::from(23.5);

    assert!(string_val.matches(&string_val_copy, &scope));
    assert!(!string_val.matches(&string_val2, &scope));

    assert!(number_val.matches(&number_val_copy, &scope));
    assert!(!number_val.matches(&number_val2, &scope));

    assert!(!string_val2.matches(&number_val2, &scope));

    assert!(!string_val2.matches(&number_val2, &scope));

    let string_cat_zero = FluentValue::from("zero");
    let string_cat_one = FluentValue::from("one");
    let string_cat_two = FluentValue::from("two");
    let string_cat_few = FluentValue::from("few");
    let string_cat_many = FluentValue::from("many");
    let string_cat_other = FluentValue::from("other");

    let number_cat_zero = 0.into();
    let number_cat_one = 1.into();
    let number_cat_two = 2.into();
    let number_cat_few = 3.into();
    let number_cat_many = 11.into();
    let number_cat_other = 101.into();

    assert!(string_cat_zero.matches(&number_cat_zero, &scope));
    assert!(string_cat_one.matches(&number_cat_one, &scope));
    assert!(string_cat_two.matches(&number_cat_two, &scope));
    assert!(string_cat_few.matches(&number_cat_few, &scope));
    assert!(string_cat_many.matches(&number_cat_many, &scope));
    assert!(string_cat_other.matches(&number_cat_other, &scope));
    assert!(!string_cat_other.matches(&number_cat_one, &scope));

    assert!(!string_val2.matches(&number_cat_one, &scope));
}

#[test]
fn fluent_value_from() {
    let value_str = FluentValue::from("my str");
    let value_string = FluentValue::from(String::from("my string"));
    let value_f64 = FluentValue::from(23.5);
    let value_isize = FluentValue::from(-23);

    assert_eq!(value_str, "my str".into());
    assert_eq!(value_string, "my string".into());

    assert_eq!(value_f64, FluentValue::from(23.5));
    assert_eq!(value_isize, FluentValue::from(-23));
}

#[test]
fn fluent_number_style() {
    let fns_decimal: FluentNumberStyle = "decimal".into();
    let fns_currency: FluentNumberStyle = "currency".into();
    let fns_percent: FluentNumberStyle = "percent".into();
    let fns_decimal2: FluentNumberStyle = "other".into();
    assert_eq!(fns_decimal, FluentNumberStyle::Decimal);
    assert_eq!(fns_currency, FluentNumberStyle::Currency);
    assert_eq!(fns_percent, FluentNumberStyle::Percent);
    assert_eq!(fns_decimal2, FluentNumberStyle::Decimal);

    let fncds_symbol: FluentNumberCurrencyDisplayStyle = "symbol".into();
    let fncds_code: FluentNumberCurrencyDisplayStyle = "code".into();
    let fncds_name: FluentNumberCurrencyDisplayStyle = "name".into();
    let fncds_symbol2: FluentNumberCurrencyDisplayStyle = "other".into();

    assert_eq!(fncds_symbol, FluentNumberCurrencyDisplayStyle::Symbol);
    assert_eq!(fncds_code, FluentNumberCurrencyDisplayStyle::Code);
    assert_eq!(fncds_name, FluentNumberCurrencyDisplayStyle::Name);
    assert_eq!(fncds_symbol2, FluentNumberCurrencyDisplayStyle::Symbol);

    let mut fno = FluentNumberOptions::default();

    let mut args = FluentArgs::new();
    args.set("style", "currency");
    args.set("currency", "EUR");
    args.set("currencyDisplay", "code");
    args.set("useGrouping", "false");
    args.set("minimumIntegerDigits", 3);
    args.set("minimumFractionDigits", 3);
    args.set("maximumFractionDigits", 8);
    args.set("minimumSignificantDigits", 1);
    args.set("maximumSignificantDigits", 10);
    args.set("someRandomOption", 10);

    fno.merge(&args);

    assert_eq!(fno.style, FluentNumberStyle::Currency);
    assert_eq!(fno.currency, Some("EUR".to_string()));
    assert_eq!(fno.currency_display, FluentNumberCurrencyDisplayStyle::Code);
    assert_eq!(fno.use_grouping, FluentNumberUseGrouping::False);

    let num = FluentNumber::new(0.2, FluentNumberOptions::default());
    assert_eq!(num.as_string_basic(), "0.2");

    let opts = FluentNumberOptions {
        minimum_fraction_digits: Some(3),
        ..Default::default()
    };

    let num = FluentNumber::new(0.2, opts.clone());
    assert_eq!(num.as_string_basic(), "0.200");

    let num = FluentNumber::new(2.0, opts);
    assert_eq!(num.as_string_basic(), "2.000");
}

#[test]
fn fluent_number_to_operands() {
    let num = FluentNumber::new(2.81, FluentNumberOptions::default());
    let operands: PluralOperands = (&num).into();

    assert_eq!(
        operands,
        PluralOperands {
            n: 2.81,
            i: 2,
            v: 2,
            w: 2,
            f: 81,
            t: 81,
        }
    );
}

#[test]
fn fluent_number_grouping() {
    let langid_ars = langid!("ccp");
    let mut bundle: FluentBundle<FluentResource> = FluentBundle::new(vec![langid_ars]);
    bundle.set_icu_data_provider(Box::new(icu_testdata::any_no_fallback()));

    let mut number = FluentNumber::from(1234567890.1234567);

    number.options.use_grouping = FluentNumberUseGrouping::False;
    let no_grouping = number.as_string(&bundle);
    assert_eq!(no_grouping, "𑄷𑄸𑄹𑄺𑄻𑄼𑄽𑄾𑄿𑄶.𑄷𑄸𑄹𑄺𑄻𑄼𑄽");

    number.options.use_grouping = FluentNumberUseGrouping::Min2;
    let long = number.as_string(&bundle);
    assert_eq!(long, "𑄷,𑄸𑄹,𑄺𑄻,𑄼𑄽,𑄾𑄿𑄶.𑄷𑄸𑄹𑄺𑄻𑄼𑄽");

    number.value = -1234.0;
    let short = number.as_string(&bundle);
    assert_eq!(short, "-𑄷𑄸𑄹𑄺");
}
