extern crate fluent;

use self::fluent::context::FluentBundle;

#[test]
fn format() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
        "
foo = Foo
    .attr = Attribute
-term = Term
",
    );

    let value = bundle.format("foo", None);
    assert_eq!(value, Some("Foo".to_string()));

    let value = bundle.format("foo.attr", None);
    assert_eq!(value, Some("Attribute".to_string()));

    let value = bundle.format("foo.missing", None);
    assert_eq!(value, None);

    let value = bundle.format("foo.attr.nested", None);
    assert_eq!(value, None);

    let value = bundle.format("missing", None);
    assert_eq!(value, None);

    let value = bundle.format("missing.attr", None);
    assert_eq!(value, None);

    let value = bundle.format("-term", None);
    assert_eq!(value, None);

    let value = bundle.format("-term.attr", None);
    assert_eq!(value, None);
}
