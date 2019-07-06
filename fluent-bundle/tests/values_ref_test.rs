mod helpers;
use fluent_bundle::errors::FluentError;
use fluent_bundle::resolve::ResolverError;

use self::helpers::{
    assert_format, assert_format_no_errors, assert_get_bundle_no_errors,
    assert_get_resource_from_str_no_errors,
};

#[test]
fn referencing_values() {
    let res = assert_get_resource_from_str_no_errors(
        r#"
key1 = Value 1
-key2 = { $sel ->
    [a] A2
   *[b] B2
}
key3 = Value { 3 }
-key4 = { $sel ->
    [a] A{ 4 }
   *[b] B{ 4 }
}
key5 =
    .a = A5
    .b = B5
ref1 = { key1 }
ref2 = { -key2 }
ref3 = { key3 }
ref4 = { -key4 }
ref5 = { key5 }
ref6 = { -key2(sel: "a") }
ref7 = { -key2(sel: "b") }
ref8 = { -key4(sel: "a") }
ref9 = { -key4(sel: "b") }
ref10 = { key5.a }
ref11 = { key5.b }
    "#,
    );
    let bundle = assert_get_bundle_no_errors(res, None);

    assert_format_no_errors(bundle.format("ref1", None), "Value 1");

    assert_format_no_errors(bundle.format("ref2", None), "B2");

    assert_format_no_errors(bundle.format("ref3", None), "Value 3");

    assert_format_no_errors(bundle.format("ref4", None), "B4");

    // XXX: Seems like a bug in JS impl because
    // it expects "???" here...
    assert_format(
        bundle.format("ref5", None),
        "key5",
        vec![FluentError::ResolverError(ResolverError::Reference(
            "Unknown message: key5".into(),
        ))],
    );

    assert_format_no_errors(bundle.format("ref6", None), "A2");
    assert_format_no_errors(bundle.format("ref7", None), "B2");

    assert_format_no_errors(bundle.format("ref8", None), "A4");
    assert_format_no_errors(bundle.format("ref9", None), "B4");

    assert_format_no_errors(bundle.format("ref10", None), "A5");
    assert_format_no_errors(bundle.format("ref11", None), "B5");
}
