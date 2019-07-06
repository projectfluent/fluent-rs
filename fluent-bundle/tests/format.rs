mod helpers;

use self::helpers::{
    assert_format_no_errors, assert_format_none, assert_get_bundle_no_errors,
    assert_get_resource_from_str_no_errors,
};

#[test]
fn format() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo = Foo
    .attr = Attribute
-term = Term
",
    );
    let bundle = assert_get_bundle_no_errors(res, None);

    assert_format_no_errors(bundle.format("foo", None), "Foo");

    assert_format_no_errors(bundle.format("foo.attr", None), "Attribute");

    assert_format_none(bundle.format("foo.missing", None));

    assert_format_none(bundle.format("foo.attr.nested", None));

    assert_format_none(bundle.format("missing", None));

    assert_format_none(bundle.format("missing.attr", None));

    assert_format_none(bundle.format("-term", None));

    assert_format_none(bundle.format("-term.attr", None));
}
