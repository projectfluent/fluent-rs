use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::resource::FluentResource;
use fluent_bundle::types::FluentValue;

fn main() {
    // We define the resources here so that they outlive
    // the bundle.
    let res1 = FluentResource::try_new("hello-world = Hey there! { HELLO() }".to_owned()).unwrap();
    let res2 =
        FluentResource::try_new("meaning-of-life = { MEANING_OF_LIFE(42) }".to_owned()).unwrap();
    let res3 = FluentResource::try_new(
        "all-your-base = { BASE_OWNERSHIP(hello, ownership: \"us\") }".to_owned(),
    )
    .unwrap();

    let mut bundle = FluentBundle::new(&["en-US"]);

    // Test for a simple function that returns a string
    bundle
        .add_function("HELLO", |_args, _named_args| {
            return Some("I'm a function!".into());
        })
        .unwrap();

    // Test for a function that accepts unnamed positional arguments
    bundle
        .add_function("MEANING_OF_LIFE", |args, _named_args| {
            if let Some(arg0) = args.get(0) {
                if *arg0 == Some(FluentValue::Number(String::from("42"))) {
                    return Some("The answer to life, the universe, and everything".into());
                }
            }

            None
        })
        .unwrap();

    // Test for a function that accepts named arguments
    bundle
        .add_function("BASE_OWNERSHIP", |_args, named_args| {
            let ownership = named_args.get("ownership").unwrap();

            return match ownership {
                &FluentValue::String(ref string) => {
                    Some(format!("All your base belong to {}", string).into())
                }
                _ => None,
            };
        })
        .unwrap();

    bundle.add_resource(&res1).unwrap();
    bundle.add_resource(&res2).unwrap();
    bundle.add_resource(&res3).unwrap();

    let value = bundle.format("hello-world", None);
    assert_eq!(
        value,
        Some(("Hey there! I'm a function!".to_string(), vec![]))
    );

    let value = bundle.format("meaning-of-life", None);
    assert_eq!(
        value,
        Some((
            "The answer to life, the universe, and everything".to_string(),
            vec![]
        ))
    );

    let (value, _) = bundle.format("all-your-base", None).unwrap();
    assert_eq!(&value, "All your base belong to us");
}
