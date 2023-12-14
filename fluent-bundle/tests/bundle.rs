use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use icu_locid::langid;
use std::borrow::Cow;

#[test]
fn add_resource_override() {
    let res = FluentResource::try_new("key = Value".to_string()).unwrap();
    let res2 = FluentResource::try_new("key = Value 2".to_string()).unwrap();

    let en_us = langid!("en-US");
    let mut bundle = FluentBundle::new(vec![en_us]);

    bundle.add_resource(&res).expect("Failed to add a resource");

    assert!(bundle.add_resource(&res2).is_err());

    let mut errors = vec![];

    let value = bundle
        .get_message("key")
        .expect("Failed to retrieve a message")
        .value()
        .expect("Failed to retrieve a value of a message");
    assert_eq!(bundle.format_pattern(value, None, &mut errors), "Value");

    bundle.add_resource_overriding(&res2);

    let value = bundle
        .get_message("key")
        .expect("Failed to retrieve a message")
        .value()
        .expect("Failed to retrieve a value of a message");
    assert_eq!(bundle.format_pattern(value, None, &mut errors), "Value 2");

    assert!(errors.is_empty());
}

#[test]
fn borrowed_plain_message() {
    let res = FluentResource::try_new("key = Value".to_string()).unwrap();
    let en_us = langid!("en-US");

    let mut bundle = FluentBundle::new(vec![en_us]);
    bundle.add_resource(&res).expect("Failed to add a resource");

    let mut errors = vec![];

    let formatted_pattern = {
        let value = bundle
            .get_message("key")
            .expect("Failed to retrieve a message")
            .value()
            .expect("Failed to retrieve a value of a message");

        bundle.format_pattern(value, None, &mut errors)
    };

    assert_eq!(formatted_pattern, "Value");
    assert!(matches!(formatted_pattern, Cow::Borrowed(_)));
}

#[test]
fn arguments_outlive_formatted_pattern() {
    let res = FluentResource::try_new("key = { $variable }".to_string()).unwrap();
    let en_us = langid!("en-US");

    let mut bundle = FluentBundle::new(vec![en_us]);
    bundle.add_resource(&res).expect("Failed to add a resource");

    let mut errors = vec![];

    let formatted_pattern = {
        let value = bundle
            .get_message("key")
            .expect("Failed to retrieve a message")
            .value()
            .expect("Failed to retrieve a value of a message");

        let mut args = FluentArgs::new();
        args.set("variable", "Variable");

        bundle.format_pattern(value, Some(&args), &mut errors)
    };

    assert_eq!(formatted_pattern, "Variable");
}
