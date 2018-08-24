extern crate fluent;

use self::fluent::context::MessageContext;

#[test]
fn format() {
    let mut ctx = MessageContext::new(&["x-testing"]);
    ctx.add_messages(
        "
foo = Foo
    .attr = Attribute
    .attr2 = Attribute 2
",
    );

    let msg = ctx.format_message("foo", None);
    assert!(msg.is_some());
    let msg = msg.unwrap();
    assert_eq!(msg.value, Some("Foo".to_string()));
    assert_eq!(msg.attributes.get("attr"), Some(&"Attribute".to_string()));
    assert_eq!(
        msg.attributes.get("attr2"),
        Some(&"Attribute 2".to_string())
    );
}
