extern crate fluent;

use fluent::context::FluentBundle;

fn main() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
        "
foo = Foo
foobar = { foo } Bar
bazbar = { baz } Bar
",
    );

    match bundle.format("foobar", None) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    match bundle.format("bazbar", None) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }
}
