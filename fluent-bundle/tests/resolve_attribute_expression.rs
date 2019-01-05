mod helpers;

use self::helpers::{
    assert_format_no_errors, assert_get_bundle_no_errors, assert_get_resource_from_str_no_errors,
};

#[test]
fn attribute_expression() {
    let res = assert_get_resource_from_str_no_errors(
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
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_format_no_errors(bundle.format("use-foo", None), "Foo");

    assert_format_no_errors(bundle.format("use-foo-attr", None), "Foo Attr");

    assert_format_no_errors(bundle.format("use-bar", None), "___");

    assert_format_no_errors(bundle.format("use-bar-attr", None), "Bar Attr");

    assert_format_no_errors(bundle.format("missing-attr", None), "___");

    assert_format_no_errors(bundle.format("missing-missing", None), "___");
}
