extern crate fluent;

use self::fluent::context::MessageContext;

#[test]
fn format_attribute() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages(
        "
foo = Foo
    .attr = Foo Attr
",
    );

    if let Some(attributes) = ctx.get_message("foo").and_then(
        |message| message.attributes.as_ref(),
    )
    {
        let value = attributes.first().and_then(
            |attribute| ctx.format(attribute, None),
        );

        assert_eq!(value, Some("Foo Attr".to_string()));
    }
}
