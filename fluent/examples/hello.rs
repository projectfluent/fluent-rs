extern crate fluent;

use fluent::FluentBundle;

fn main() {
    let mut bundle = FluentBundle::new(&["en-US"]);
    bundle.add_messages("hello-world = Hello, world!").unwrap();
    let value = bundle.format("hello-world", None).unwrap();
    assert_eq!(&value.0, "Hello, world!");
}
