use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::resource::FluentResource;

fn main() {
    let res = FluentResource::try_new(
        "
foo = Foo
foobar = { foo } Bar
bazbar = { baz } Bar
"
        .to_owned(),
    )
    .unwrap();

    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_resource(&res).unwrap();

    match bundle.format("foobar", None) {
        Some((value, _)) => println!("{}", value),
        _ => println!("None"),
    }

    match bundle.format("bazbar", None) {
        Some((value, _)) => println!("{}", value),
        _ => println!("None"),
    }
}
