mod helpers;

use self::helpers::{
    assert_format_no_errors, assert_get_bundle_no_errors, assert_get_resource_from_str_no_errors,
};

#[test]
fn format_message() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo = Foo
    ",
    );
    let bundle = assert_get_bundle_no_errors(res, None);

    assert_format_no_errors(bundle.format("foo", None), "Foo");
}

#[test]
fn format_attribute() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo = Foo
    .attr = Foo Attr
    ",
    );
    let bundle = assert_get_bundle_no_errors(res, None);

    assert_format_no_errors(bundle.format("foo.attr", None), "Foo Attr");
}
