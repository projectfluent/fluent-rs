mod helpers;
use fluent_bundle::errors::FluentError;
use fluent_bundle::resolve::ResolverError;

use self::helpers::{
    assert_format, assert_format_no_errors, assert_get_bundle_no_errors,
    assert_get_resource_from_str_no_errors,
};

#[test]
fn formatting_values() {
    let res = assert_get_resource_from_str_no_errors(
        "
key1 = Value
-term1 = Value
  .attr = Value
key2 = Value { -term1 }
key3 = Value { -term1.attr ->
    [Value] Foo
   *[other] Faa
}
key4 = Value { 4 }

key5 = Value { 4 ->
    [4] Foo
   *[other] Faa
}
key6 = Value { key11000 }
key7 = Value { -key11000 }
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_format_no_errors(bundle.format("key1", None), "Value");

    assert_format_no_errors(bundle.format("key2", None), "Value Value");
    assert_format_no_errors(bundle.format("key3", None), "Value Foo");
    assert_format_no_errors(bundle.format("key4", None), "Value 4");
    assert_format_no_errors(bundle.format("key5", None), "Value Foo");
    assert_format(
        bundle.format("key6", None),
        "Value key11000",
        vec![FluentError::ResolverError(ResolverError::None)],
    );
    assert_format(
        bundle.format("key7", None),
        "Value -key11000",
        vec![FluentError::ResolverError(ResolverError::None)],
    );
}
