use fluent_bundle::{FluentBundle, FluentResource};

fn main() {
    let ftl_string = String::from("hello-world = Hello, world!");
    let res = FluentResource::try_new(ftl_string).expect("Could not parse an FTL string.");
    let mut bundle = FluentBundle::default();
    bundle
        .add_resource(&res)
        .expect("Failed to add FTL resources to the bundle.");

    let msg = bundle
        .get_message("hello-world")
        .expect("Message doesn't exist.");
    let pattern = msg.value.expect("Message has no value.");
    let (value, _) = bundle.format_pattern(&pattern, None);
    assert_eq!(&value, "Hello, world!");
}
