extern crate fluent;

use fluent::FluentBundle;

fn main() {
    let mut bundle = FluentBundle::new(&["en-US"]);
    bundle.add_messages("hello-world = Hello, world!");

    let value = bundle.format("hello-world", None);
    assert_eq!(value, Some("Hello, world!".to_string()));
}
