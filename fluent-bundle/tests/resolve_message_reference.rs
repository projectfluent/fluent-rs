mod helpers;

use self::helpers::{assert_add_messages_no_errors, assert_format_no_errors};
use fluent_bundle::bundle::FluentBundle;

#[test]
fn message_reference() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages(
        "
foo = Foo
bar = { foo } Bar
",
    ));

    assert_format_no_errors(bundle.format("bar", None), "Foo Bar");
}

#[test]
fn term_reference() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages(
        "
-foo = Foo
bar = { -foo } Bar
",
    ));

    assert_format_no_errors(bundle.format("bar", None), "Foo Bar");
}

#[test]
fn message_reference_nested() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages(
        "
foo = Foo
bar = { foo } Bar
baz = { bar } Baz
",
    ));

    assert_format_no_errors(bundle.format("baz", None), "Foo Bar Baz");
}

#[test]
fn message_reference_missing() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages("bar = { foo } Bar"));

    assert_format_no_errors(bundle.format("bar", None), "___ Bar");
}

#[test]
fn message_reference_cyclic() {
    {
        let mut bundle = FluentBundle::new(&["x-testing"]);
        assert_add_messages_no_errors(bundle.add_messages(
            "
foo = Foo { bar }
bar = { foo } Bar
",
        ));

        assert_format_no_errors(bundle.format("foo", None), "Foo ___");
        assert_format_no_errors(bundle.format("bar", None), "___ Bar");
    }

    {
        let mut bundle = FluentBundle::new(&["x-testing"]);
        assert_add_messages_no_errors(bundle.add_messages(
            "
foo = { bar }
bar = { foo }
",
        ));

        assert_format_no_errors(bundle.format("foo", None), "___");
        assert_format_no_errors(bundle.format("bar", None), "___");
    }
}

#[test]
fn message_reference_multiple() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages(
        "
foo = Foo
bar = { foo } Bar { foo }
",
    ));

    assert_format_no_errors(bundle.format("bar", None), "Foo Bar Foo");
}
