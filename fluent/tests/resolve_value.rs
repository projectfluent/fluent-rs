extern crate fluent;

use self::fluent::context::MessageContext;

#[test]
fn format_message() {
    let mut ctx = MessageContext::new(&["x-testing"]);
    ctx.add_messages(
        "
foo = Foo
",
    );

    let value = ctx.format("foo", None);
    assert_eq!(value, Some("Foo".to_string()));
}

#[test]
fn format_attribute() {
    let mut ctx = MessageContext::new(&["x-testing"]);
    ctx.add_messages(
        "
foo = Foo
    .attr = Foo Attr
",
    );

    let value = ctx.format("foo.attr", None);
    assert_eq!(value, Some("Foo Attr".to_string()));
}
