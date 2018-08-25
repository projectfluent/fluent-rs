extern crate fluent;

use self::fluent::context::FluentBundle;

#[test]
fn message_reference() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
        "
foo = Foo
bar = { foo } Bar
",
    );

    let value = bundle.format("bar", None);
    assert_eq!(value, Some("Foo Bar".to_string()));
}

#[test]
fn term_reference() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
        "
-foo = Foo
bar = { -foo } Bar
",
    );

    let value = bundle.format("bar", None);
    assert_eq!(value, Some("Foo Bar".to_string()));
}

#[test]
fn message_reference_nested() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
        "
foo = Foo
bar = { foo } Bar
baz = { bar } Baz
",
    );

    let value = bundle.format("baz", None);
    assert_eq!(value, Some("Foo Bar Baz".to_string()));
}

#[test]
fn message_reference_missing() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages("bar = { foo } Bar");

    let value = bundle.format("bar", None);
    assert_eq!(value, Some("___ Bar".to_string()));
}

#[test]
fn message_reference_cyclic() {
    {
        let mut bundle = FluentBundle::new(&["x-testing"]);
        bundle.add_messages(
            "
foo = Foo { bar }
bar = { foo } Bar
",
        );

        let value = bundle.format("foo", None);
        assert_eq!(value, Some("Foo ___".to_string()));
        let value = bundle.format("bar", None);
        assert_eq!(value, Some("___ Bar".to_string()));
    }

    {
        let mut bundle = FluentBundle::new(&["x-testing"]);
        bundle.add_messages(
            "
foo = { bar }
bar = { foo }
",
        );

        let value = bundle.format("foo", None);
        assert_eq!(value, Some("___".to_string()));
        let value = bundle.format("bar", None);
        assert_eq!(value, Some("___".to_string()));
    }
}

#[test]
fn message_reference_multiple() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
        "
foo = Foo
bar = { foo } Bar { foo }
",
    );

    let value = bundle.format("bar", None);
    assert_eq!(value, Some("Foo Bar Foo".to_string()));
}
