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

    let value = ctx.format("use-foo", None);
    assert_eq!(value, Some("Foo".to_string()));

    let value = ctx.format("use-foo-attr", None);
    assert_eq!(value, Some("Foo Attr".to_string()));

    let value = ctx.format("use-bar", None);
    assert_eq!(value, Some("___".to_string()));

    let value = ctx.format("use-bar-attr", None);
    assert_eq!(value, Some("Bar Attr".to_string()));

    let value = ctx.format("missing-attr", None);
    assert_eq!(value, Some("___".to_string()));

    let value = ctx.format("missing-missing", None);
    assert_eq!(value, Some("___".to_string()));
}
