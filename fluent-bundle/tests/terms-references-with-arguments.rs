use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};

#[test]
fn test_term_argument_resolve() {
    // 1. Create bundle
    let ftl_string = String::from(
        "
-liked-count = { $count ->
        [0]     No likes yet.
        [one]   One person liked your message
       *[other] { $count } people liked your message
}

annotation = Beautiful! { -liked-count(count: $num) }
    ",
    );

    let res = FluentResource::try_new(ftl_string).expect("Could not parse an FTL string.");
    let mut bundle = FluentBundle::default();

    bundle
        .add_function("NUMBER", |positional, named| match positional.first() {
            Some(FluentValue::Number(n)) => {
                let mut num = n.clone();
                num.options.merge(named);

                FluentValue::Number(num)
            }
            _ => FluentValue::Error,
        })
        .expect("Failed to add a function.");

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    // 1. Example with passing custom argument to term
    let mut args = FluentArgs::new();
    args.set("num", FluentValue::from(1));

    let msg = bundle
        .get_message("annotation")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("Beautiful! One person liked your message", &value);

    let mut args = FluentArgs::new();
    args.set("num", FluentValue::from(5));

    let msg = bundle
        .get_message("annotation")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!(
        "Beautiful! \u{2068}5\u{2069} people liked your message",
        &value
    );

    let mut args = FluentArgs::new();
    args.set("num", FluentValue::from(0));

    let msg = bundle
        .get_message("annotation")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("Beautiful! No likes yet.", &value);
}
