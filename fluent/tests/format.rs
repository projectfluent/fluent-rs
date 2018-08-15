extern crate fluent;

use self::fluent::context::MessageContext;

#[test]
fn format() {
    let mut ctx = MessageContext::new(&["x-testing"]);
    ctx.add_messages(
        "
foo = Foo
    .attr = Attribute
-term = Term
",
    );

    let value = ctx.format("foo", None);
    assert_eq!(value, Some("Foo".to_string()));

    let value = ctx.format("foo.attr", None);
    assert_eq!(value, Some("Attribute".to_string()));

    let value = ctx.format("foo.missing", None);
    assert_eq!(value, None);

    let value = ctx.format("foo.attr.nested", None);
    assert_eq!(value, None);

    let value = ctx.format("missing", None);
    assert_eq!(value, None);

    let value = ctx.format("missing.attr", None);
    assert_eq!(value, None);

    let value = ctx.format("-term", None);
    assert_eq!(value, None);

    let value = ctx.format("-term.attr", None);
    assert_eq!(value, None);
}
