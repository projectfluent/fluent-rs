use fluent_bundle::{FluentBundle, FluentResource};

fn main() {
    let ftl_string = String::from("hello-world = Hello, world!");
    let res = FluentResource::try_new(ftl_string).expect("Could not parse an FTL string.");
    let mut bundle = FluentBundle::new(&["en-US"]);
    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");
    let (value, _) = bundle
        .format("hello-world", None)
        .expect("Failed to format a message.");
    assert_eq!(&value, "Hello, world!");
}
