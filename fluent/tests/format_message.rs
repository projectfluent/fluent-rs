extern crate fluent;

mod helpers;

use fluent::context::FluentBundle;
use fluent::context::Message;
use helpers::{assert_add_messages_no_errors, assert_format_message_no_errors};
use std::collections::HashMap;

#[test]
fn format() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages(
        "
foo = Foo
    .attr = Attribute
    .attr2 = Attribute 2
",
    ));

    let mut attrs = HashMap::new();
    attrs.insert("attr".to_string(), "Attribute".to_string());
    attrs.insert("attr2".to_string(), "Attribute 2".to_string());

    assert_format_message_no_errors(
        bundle.format_message("foo", None),
        Message {
            value: Some("Foo".to_string()),
            attributes: attrs,
        },
    );
}
