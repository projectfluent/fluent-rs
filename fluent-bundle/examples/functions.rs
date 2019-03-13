use fluent_bundle::{FluentBundle, FluentResource, FluentValue};

fn main() {
    // We define the resources here so that they outlive
    // the bundle.
    let ftl_string1 = String::from("hello-world = Hey there! { HELLO() }");
    let ftl_string2 = String::from("meaning-of-life = { MEANING_OF_LIFE(42) }");
    let ftl_string3 = String::from("all-your-base = { BASE_OWNERSHIP(hello, ownership: \"us\") }");
    let res1 = FluentResource::try_new(ftl_string1).expect("Could not parse an FTL string.");
    let res2 = FluentResource::try_new(ftl_string2).expect("Could not parse an FTL string.");
    let res3 = FluentResource::try_new(ftl_string3).expect("Could not parse an FTL string.");

    let mut bundle = FluentBundle::new(&["en-US"]);

    // Test for a simple function that returns a string
    bundle
        .add_function("HELLO", |_args, _named_args| {
            return "I'm a function!".into();
        })
        .expect("Failed to add a function to the bundle.");

    // Test for a function that accepts unnamed positional arguments
    bundle
        .add_function("MEANING_OF_LIFE", |args, _named_args| {
            if let Some(arg0) = args.get(0) {
                if *arg0 == FluentValue::Number("42".into()) {
                    return "The answer to life, the universe, and everything".into();
                }
            }

            FluentValue::None(None)
        })
        .expect("Failed to add a function to the bundle.");

    // Test for a function that accepts named arguments
    bundle
        .add_function("BASE_OWNERSHIP", |_args, named_args| {
            return match named_args.get("ownership") {
                Some(FluentValue::String(ref string)) => {
                    format!("All your base belong to {}", string).into()
                }
                _ => FluentValue::None(None),
            };
        })
        .expect("Failed to add a function to the bundle.");

    bundle
        .add_resource(&res1)
        .expect("Failed to add FTL resources to the bundle.");
    bundle
        .add_resource(&res2)
        .expect("Failed to add FTL resources to the bundle.");
    bundle
        .add_resource(&res3)
        .expect("Failed to add FTL resources to the bundle.");

    let (value, _) = bundle
        .format("hello-world", None)
        .expect("Failed to format a message.");
    assert_eq!(&value, "Hey there! I'm a function!");

    let (value, _) = bundle
        .format("meaning-of-life", None)
        .expect("Failed to format a message.");
    assert_eq!(&value, "The answer to life, the universe, and everything");

    let (value, _) = bundle
        .format("all-your-base", None)
        .expect("Failed to format a message.");
    assert_eq!(&value, "All your base belong to us");
}
