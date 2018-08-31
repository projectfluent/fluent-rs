extern crate fluent;

use fluent::bundle::FluentBundle;

fn main() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle
        .add_messages(
            "
foo = Foo
foobar = { foo } Bar
bazbar = { baz } Bar
",
        ).unwrap();

    match bundle.format("foobar", None) {
        Some((value, _)) => println!("{}", value),
        _ => println!("None"),
    }

    match bundle.format("bazbar", None) {
        Some((value, _)) => println!("{}", value),
        _ => println!("None"),
    }
}
