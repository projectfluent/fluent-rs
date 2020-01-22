use fluent_bundle::resolve::Scope;
use fluent_bundle::types::FluentType;
use fluent_bundle::FluentBundle;
use fluent_bundle::FluentResource;
use fluent_bundle::FluentValue;
use unic_langid::langid;

#[test]
fn fluent_value_number() {
    let value = FluentValue::into_number("invalid");
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

    let number_val = FluentValue::into_number("-23.5");
    let number_val_copy = FluentValue::into_number("-23.5");
    let number_val2 = FluentValue::into_number("23.5");

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

    let number_cat_zero = FluentValue::into_number("0");
    let number_cat_one = FluentValue::into_number("1");
    let number_cat_two = FluentValue::into_number("2");
    let number_cat_few = FluentValue::into_number("3");
    let number_cat_many = FluentValue::into_number("11");
    let number_cat_other = FluentValue::into_number("101");

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

    assert_eq!(value_f64, FluentValue::into_number("23.5"));
    assert_eq!(value_isize, FluentValue::into_number("-23"));
}

#[test]
fn fluent_custom_type() {
    #[derive(Debug, PartialEq)]
    struct DateTime {
        epoch: usize,
    };

    impl DateTime {
        pub fn new(epoch: usize) -> Self {
            Self { epoch }
        }
    }

    impl FluentType for DateTime {
        fn duplicate(&self) -> Box<dyn FluentType> {
            Box::new(DateTime { epoch: self.epoch })
        }
        fn as_string(&self) -> std::borrow::Cow<'static, str> {
            format!("{}", self.epoch).into()
        }
    }

    impl std::fmt::Display for DateTime {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.epoch)
        }
    }

    let dt = FluentValue::Custom(Box::new(DateTime::new(10)));
    let dt2 = FluentValue::Custom(Box::new(DateTime::new(10)));
    let dt3 = FluentValue::Custom(Box::new(DateTime::new(15)));

    let sv = FluentValue::String("foo".into());

    assert_eq!(dt == dt2, true);
    assert_eq!(dt == dt3, false);
    assert_eq!(dt == sv, false);
}

#[test]
fn fluent_custom_type_format() {
    #[derive(Debug, PartialEq)]
    struct DateTime {
        epoch: usize,
    };

    impl DateTime {
        pub fn new(epoch: usize) -> Self {
            Self { epoch }
        }
    }

    impl FluentType for DateTime {
        fn duplicate(&self) -> Box<dyn FluentType> {
            Box::new(DateTime { epoch: self.epoch })
        }
        fn as_string(&self) -> std::borrow::Cow<'static, str> {
            format!("{}", self.epoch).into()
        }
    }

    impl std::fmt::Display for DateTime {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.epoch)
        }
    }

    fn custom_formatter(num: &FluentValue) -> Option<String> {
        match num {
            FluentValue::Number(n, mfd) => {
                Some(format!("{}.{}", n, "0".repeat(mfd.unwrap_or(2))).into())
            }
            _ => None,
        }
    }

    let res = FluentResource::try_new(
        r#"
key-num-implicit = Hello { 5.000 } World
key-num-explicit = Hello { NUMBER(5, minimumFractionDigits: 2) } World
    "#
        .into(),
    )
    .unwrap();
    let mut bundle = FluentBundle::default();
    bundle.add_resource(res).unwrap();
    bundle.set_use_isolating(false);

    bundle.set_formatter(Some(custom_formatter));

    bundle
        .add_function("NUMBER", |positional, named| match positional {
            [FluentValue::Number(n, mfd)] => {
                let mfd = if let Some(mfd) = named.get("minimumFractionDigits") {
                    if let FluentValue::Number(v, _) = mfd {
                        Some(*v as usize)
                    } else {
                        None
                    }
                } else {
                    mfd.clone()
                };
                FluentValue::Number(n.clone(), mfd)
            }
            _ => FluentValue::None,
        })
        .unwrap();

    let mut errors = vec![];

    let msg = bundle.get_message("key-num-explicit").unwrap();
    let val = bundle.format_pattern(msg.value.unwrap(), None, &mut errors);
    assert_eq!(val, "Hello 5.00 World");

    let msg = bundle.get_message("key-num-implicit").unwrap();
    let val = bundle.format_pattern(msg.value.unwrap(), None, &mut errors);
    assert_eq!(val, "Hello 5.000 World");
}
