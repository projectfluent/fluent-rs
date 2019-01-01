mod helpers;

use self::helpers::{assert_add_messages_no_errors, assert_format_no_errors};
use fluent::bundle::FluentBundle;

#[test]
fn attribute_expression() {
    let mut bundle = FluentBundle::new(&["x-testing"], None);

    assert_add_messages_no_errors(bundle.add_messages(
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
    ));

    assert_format_no_errors(bundle.format("use-foo", None), "Foo");

    assert_format_no_errors(bundle.format("use-foo-attr", None), "Foo Attr");

    assert_format_no_errors(bundle.format("use-bar", None), "___");

    assert_format_no_errors(bundle.format("use-bar-attr", None), "Bar Attr");

    assert_format_no_errors(bundle.format("missing-attr", None), "___");

    assert_format_no_errors(bundle.format("missing-missing", None), "___");
}
