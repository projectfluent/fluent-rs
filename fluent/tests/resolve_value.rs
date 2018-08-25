extern crate fluent;

use self::fluent::context::FluentBundle;

#[test]
fn format_message() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
        "
foo = Foo
",
    );

    let value = bundle.format("foo", None);
    assert_eq!(value, Some("Foo".to_string()));
}

#[test]
fn format_attribute() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
        "
foo = Foo
    .attr = Foo Attr
",
    );

    let value = bundle.format("foo.attr", None);
    assert_eq!(value, Some("Foo Attr".to_string()));
}
