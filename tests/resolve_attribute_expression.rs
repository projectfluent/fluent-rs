extern crate fluent;

use self::fluent::context::MessageContext;

#[test]
fn attribute_expression() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages(
        "
foo = Foo
    .attr = Foo Attr
bar =
    .attr = Bar Attr

use-foo = { foo }
use-foo-attr = { foo.attr }
use-bar = { bar }
use-bar-attr = { bar.attr }

missing-attr = { foo.missing }
missing-missing = { missing.missing }
",
    );

    let value = ctx.get_message("use-foo")
        .and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Foo".to_string()));

    let value = ctx.get_message("use-foo-attr")
        .and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Foo Attr".to_string()));

    let value = ctx.get_message("use-bar")
        .and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("___".to_string()));

    let value = ctx.get_message("use-bar-attr")
        .and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Bar Attr".to_string()));

    let value = ctx.get_message("missing-attr")
        .and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("___".to_string()));

    let value = ctx.get_message("missing-missing")
        .and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("___".to_string()));
}
