extern crate fluent;

mod helpers;

use self::fluent::bundle::FluentBundle;
use helpers::{assert_add_messages_no_errors, assert_format_no_errors};

#[test]
fn format_message() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages(
        "
foo = Foo
",
    ));

    assert_format_no_errors(bundle.format("foo", None), "Foo");
}

#[test]
fn format_attribute() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages(
        "
foo = Foo
    .attr = Foo Attr
",
    ));

    assert_format_no_errors(bundle.format("foo.attr", None), "Foo Attr");
}
