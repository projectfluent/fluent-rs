extern crate fluent;

use self::fluent::context::MessageContext;

#[test]
fn variant_expression() {
    let mut ctx = MessageContext::new(&["x-testing"]);
    ctx.add_messages(
        "
-foo = Foo
-bar =
    {
       *[nominative] Bar
        [genitive] Bar's
    }
bar = { -bar }

use-foo = { -foo }
use-foo-missing = { -foo[missing] }

use-bar = { -bar }
use-bar-nominative = { -bar[nominative] }
use-bar-genitive = { -bar[genitive] }
use-bar-missing = { -bar[missing] }

missing-missing = { -missing[missing] }
",
    );

    let value = ctx.format("bar", None);
    assert_eq!(value, Some("Bar".to_string()));

    let value = ctx.format("use-foo", None);
    assert_eq!(value, Some("Foo".to_string()));

    let value = ctx.format("use-foo-missing", None);
    assert_eq!(value, Some("Foo".to_string()));

    let value = ctx.format("use-bar", None);
    assert_eq!(value, Some("Bar".to_string()));

    let value = ctx.format("use-bar-nominative", None);
    assert_eq!(value, Some("Bar".to_string()));

    let value = ctx.format("use-bar-genitive", None);
    assert_eq!(value, Some("Bar's".to_string()));

    let value = ctx.format("use-bar-missing", None);
    assert_eq!(value, Some("Bar".to_string()));

    let value = ctx.format("missing-missing", None);
    assert_eq!(value, Some("___".to_string()));
}
