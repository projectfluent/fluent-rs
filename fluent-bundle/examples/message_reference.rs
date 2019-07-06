use fluent_bundle::{FluentBundle, FluentResource};

fn main() {
    let ftl_string = String::from(
        "
foo = Foo
foobar = { foo } Bar
bazbar = { baz } Bar
    ",
    );
    let res = FluentResource::try_new(ftl_string).expect("Could not parse an FTL string.");

    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    match bundle.format("foobar", None) {
        Some((value, _)) => println!("{}", value),
        _ => println!("None"),
    }

    match bundle.format("bazbar", None) {
        Some((value, _)) => println!("{}", value),
        _ => println!("None"),
    }
}
