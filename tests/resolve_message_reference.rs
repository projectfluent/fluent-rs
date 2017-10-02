extern crate fluent;

use self::fluent::context::MessageContext;

#[test]
fn message_reference() {
    let mut ctx = MessageContext::new("x-testing");

    ctx.add_messages(
        "
foo = Foo
bar = { foo } Bar
",
    );

    let value = ctx.get_message("bar").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Foo Bar".to_string()));
}

#[test]
fn message_reference_nested() {
    let mut ctx = MessageContext::new("x-testing");

    ctx.add_messages(
        "
foo = Foo
bar = { foo } Bar
baz = { bar } Baz
",
    );

    let value = ctx.get_message("baz").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Foo Bar Baz".to_string()));
}

#[test]
fn message_reference_missing() {
    let mut ctx = MessageContext::new("x-testing");

    ctx.add_messages("bar = { foo } Bar");

    let value = ctx.get_message("bar").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("___ Bar".to_string()));
}
