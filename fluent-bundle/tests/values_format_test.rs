mod helpers;
use fluent_bundle::errors::FluentError;
use fluent_bundle::resolve::ResolverError;

use self::helpers::{
    assert_format, assert_format_no_errors, assert_format_none, assert_get_bundle_no_errors,
    assert_get_resource_from_str_no_errors,
};

#[test]
fn formatting_values() {
    let res = assert_get_resource_from_str_no_errors(
        "
key1 = Value 1
key2 = { $sel ->
    [a] A2
   *[b] B2
}
key3 = Value { 3 }
key4 = { $sel ->
    [a] A{ 4 }
   *[b] B{ 4 }
}
key5 =
    .a = A5
    .b = B5
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_format_no_errors(bundle.format("key1", None), "Value 1");

    assert_format(
        bundle.format("key2", None),
        "B2",
        vec![FluentError::ResolverError(ResolverError::Reference(
            "Unknown variable: $sel".into(),
        ))],
    );

    assert_format_no_errors(bundle.format("key3", None), "Value 3");

    assert_format(
        bundle.format("key4", None),
        "B4",
        vec![FluentError::ResolverError(ResolverError::Reference(
            "Unknown variable: $sel".into(),
        ))],
    );

    assert_format_none(bundle.format("key5", None));

    assert_format_no_errors(bundle.format("key5.a", None), "A5");
    assert_format_no_errors(bundle.format("key5.b", None), "B5");
}
