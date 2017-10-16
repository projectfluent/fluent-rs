extern crate fluent;

use self::fluent::context::MessageContext;

#[test]
fn variant_expression() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages(
        "
foo = Foo
bar =
    {
       *[nominative] Bar
        [genitive] Bar's
    }

use-foo = { foo }
use-foo-missing = { foo[missing] }

use-bar = { bar }
use-bar-nominative = { bar[nominative] }
use-bar-genitive = { bar[genitive] }
use-bar-missing = { bar[missing] }

missing-missing = { missing[missing] }
",
    );

    let value = ctx.get_message("bar").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Bar".to_string()));

    let value = ctx.get_message("use-foo")
        .and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Foo".to_string()));

    let value = ctx.get_message("use-foo-missing")
        .and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Foo".to_string()));

    let value = ctx.get_message("use-bar")
        .and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Bar".to_string()));

    let value = ctx.get_message("use-bar-nominative")
        .and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Bar".to_string()));

    let value = ctx.get_message("use-bar-genitive")
        .and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Bar's".to_string()));

    let value = ctx.get_message("use-bar-missing")
        .and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Bar".to_string()));

    let value = ctx.get_message("missing-missing")
        .and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("___".to_string()));
}
