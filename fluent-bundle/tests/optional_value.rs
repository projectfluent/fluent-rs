use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};

#[test]
fn test_optional_value() {
    let ftl_string = String::from(
        "
hello = { $title ->
        [Miss] Hello, Miss. { $name }!
        [Mr] Hello, Mr. { $name }!
        [Mrs] Hello, Mrs. { $name }!
        [Ms] Hello, Ms. { $name }!
       *[Mx] Hello, Mx. { $name }!
    }
    ",
    );

    let res = FluentResource::try_new(ftl_string).expect("Could not parse an FTL string.");
    let mut bundle = FluentBundle::default();
    bundle.set_use_isolating(false);

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    let msg = bundle.get_message("hello").expect("Message doesn't exist.");

    let pattern = msg.value().expect("Message has no value.");

    // Optional value that matches a non-default variant
    let mut args = FluentArgs::new();
    let title = Some("Mr");
    args.set("title", title);
    args.set("name", "John");

    let mut errors = vec![];
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("Hello, Mr. John!", &value);

    // No value, use default variant
    let mut args = FluentArgs::new();
    let title: Option<&str> = None;
    args.set("title", title);
    args.set("name", "John");

    let mut errors = vec![];
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("Hello, Mx. John!", &value);

    // Optional value that does not match any variant and therefore reverts to the default variant
    let mut args = FluentArgs::new();
    let title = Some(2);
    args.set("title", title);
    args.set("name", "John");

    let mut errors = vec![];
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("Hello, Mx. John!", &value);
}
