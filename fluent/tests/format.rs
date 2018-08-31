extern crate fluent;

mod helpers;

use self::fluent::bundle::FluentBundle;
use helpers::{assert_add_messages_no_errors, assert_format_no_errors, assert_format_none};

#[test]
fn format() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages(
        "
foo = Foo
    .attr = Attribute
-term = Term
",
    ));

    assert_format_no_errors(bundle.format("foo", None), "Foo");

    assert_format_no_errors(bundle.format("foo.attr", None), "Attribute");

    assert_format_none(bundle.format("foo.missing", None));

    assert_format_none(bundle.format("foo.attr.nested", None));

    assert_format_none(bundle.format("missing", None));

    assert_format_none(bundle.format("missing.attr", None));

    assert_format_none(bundle.format("-term", None));

    assert_format_none(bundle.format("-term.attr", None));
}
