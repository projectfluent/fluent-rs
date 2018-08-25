extern crate fluent;

use self::fluent::context::FluentBundle;

#[test]
fn variant_expression() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
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

    let value = bundle.format("bar", None);
    assert_eq!(value, Some("Bar".to_string()));

    let value = bundle.format("use-foo", None);
    assert_eq!(value, Some("Foo".to_string()));

    let value = bundle.format("use-foo-missing", None);
    assert_eq!(value, Some("Foo".to_string()));

    let value = bundle.format("use-bar", None);
    assert_eq!(value, Some("Bar".to_string()));

    let value = bundle.format("use-bar-nominative", None);
    assert_eq!(value, Some("Bar".to_string()));

    let value = bundle.format("use-bar-genitive", None);
    assert_eq!(value, Some("Bar's".to_string()));

    let value = bundle.format("use-bar-missing", None);
    assert_eq!(value, Some("Bar".to_string()));

    let value = bundle.format("missing-missing", None);
    assert_eq!(value, Some("___".to_string()));
}
