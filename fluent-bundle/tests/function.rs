use fluent_bundle::types::FluentNumber;
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};

#[test]
fn test_function_resolve() {
    // 1. Create bundle
    let ftl_string = String::from(
        "
liked-count = { $num ->
        [0]     No likes yet.
        [one]   One person liked your message
       *[other] { $num } people liked your message
    }

liked-count2 = { NUMBER($num) ->
        [0]     No likes yet.
        [one]   One person liked your message
       *[other] { $num } people liked your message
    }
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
            Some(FluentValue::String(s)) => {
                let num: f64 = if let Ok(n) = s.clone().parse() {
                    n
                } else {
                    return FluentValue::Error;
                };
                let mut num = FluentNumber {
                    value: num,
                    options: Default::default(),
                };
                num.options.merge(named);

                FluentValue::Number(num)
            }
            _ => FluentValue::Error,
        })
        .expect("Failed to add a function.");

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    // 2. Example without NUMBER call
    let msg = bundle
        .get_message("liked-count")
        .expect("Message doesn't exist.");

    let mut args = FluentArgs::new();
    args.set("num", FluentValue::from("1"));

    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("\u{2068}1\u{2069} people liked your message", &value);

    // 3. Example with passing number, but without NUMBER call
    let mut args = FluentArgs::new();
    args.set("num", FluentValue::from(1));

    let msg = bundle
        .get_message("liked-count")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("One person liked your message", &value);

    // 4. Example with NUMBER call
    let mut args = FluentArgs::new();
    args.set("num", FluentValue::from("1"));

    let msg = bundle
        .get_message("liked-count2")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("One person liked your message", &value);

    // 5. Example with NUMBER call from number
    let mut args = FluentArgs::new();
    args.set("num", FluentValue::from(1));

    let msg = bundle
        .get_message("liked-count2")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("One person liked your message", &value);
}

#[test]
fn test_extended_function() {
    struct ManualMessageReference;

    impl fluent_bundle::FluentFunctionObject for ManualMessageReference {
        fn call<'bundle>(
            &self,
            scope: &mut dyn fluent_bundle::FluentFunctionScope<'bundle>,
            positional: &[FluentValue<'bundle>],
            named: &FluentArgs<'bundle>,
        ) -> FluentValue<'bundle> {
            let Some(FluentValue::String(name)) = positional.first().cloned() else {
                return FluentValue::Error;
            };

            let Some(msg) = scope.get_message(&name) else {
                return FluentValue::Error;
            };

            let pattern = if let Some(FluentValue::String(attribute)) = positional.get(1) {
                let Some(pattern) = msg.get_attribute(attribute) else {
                    return FluentValue::Error;
                };
                Some(pattern.value())
            } else {
                msg.value()
            };

            let Some(pattern) = pattern else {
                return FluentValue::Error;
            };

            scope.format_message(pattern, Some(named.clone()))
        }
    }

    // Create bundle
    let ftl_string = String::from(
        r#"
hero-1 = Aurora
    .gender = feminine

hero-2 = Rick
    .gender = masculine

creature-horse = { $count ->
    *[one] a horse
    [other] { $count } horses
}

creature-rabbit = { $count ->
    *[one] a rabbit
    [other] { $count } rabbits
}

annotation = Beautiful! { MSGREF($creature, count: $count) }

hero-owns-creature =
    { MSGREF($hero) } arrived! 
    { MSGREF($hero, "gender") ->
        [feminine] She owns
        [masculine] He owns
        *[other] They own
    }
    { MSGREF($creature, count: $count) }

"#,
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
        .add_function_with_scope("MSGREF", ManualMessageReference)
        .expect("Failed to add a function");

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    // Examples with passing message reference to a function
    let mut args = FluentArgs::new();
    args.set("creature", FluentValue::from("creature-horse"));
    args.set("count", FluentValue::from(1));

    let msg = bundle
        .get_message("annotation")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("Beautiful! \u{2068}a horse\u{2069}", &value);

    let mut args = FluentArgs::new();
    args.set("creature", FluentValue::from("creature-rabbit"));
    args.set("count", FluentValue::from(5));

    let msg = bundle
        .get_message("annotation")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!(
        "Beautiful! \u{2068}\u{2068}5\u{2069} rabbits\u{2069}",
        &value
    );

    // Example with accessing message attributes
    let mut args = FluentArgs::new();
    args.set("hero", FluentValue::from("hero-2"));
    args.set("creature", FluentValue::from("creature-rabbit"));
    args.set("count", FluentValue::from(3));

    let msg = bundle
        .get_message("hero-owns-creature")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!(
        "\u{2068}Rick\u{2069} arrived! \n\u{2068}He owns\u{2069}\n\u{2068}\u{2068}3\u{2069} rabbits\u{2069}",
        &value
    );
}
