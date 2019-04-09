mod helpers;

use self::helpers::{
    assert_format_no_errors, assert_get_bundle_no_errors, assert_get_resource_from_str_no_errors,
};
use fluent_bundle::entry::GetEntry;
use fluent_bundle::errors::FluentError;

// XXX: We skip addMessages because we
// don't support that API in Rust.

#[test]
fn bundle_add_resource() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo = Foo
-bar = Bar
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert!(bundle.entries.get_message("foo").is_some());
    assert!(bundle.entries.get_term("foo").is_none());
    assert!(bundle.entries.get_message("bar").is_none());
    assert!(bundle.entries.get_term("bar").is_some());
}

#[test]
fn bundle_allow_overrides_false() {
    let res = assert_get_resource_from_str_no_errors(
        "
key = Foo
    ",
    );
    let res2 = assert_get_resource_from_str_no_errors(
        "
key = Bar
    ",
    );
    let mut bundle = assert_get_bundle_no_errors(&res, None);
    assert_eq!(
        bundle.add_resource(&res2),
        Err(vec![FluentError::Overriding {
            kind: "message".into(),
            id: "key".into()
        }])
    );

    assert_format_no_errors(bundle.format("key", None), "Foo");
}

#[test]
fn bundle_has_message() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo = Foo
bar =
    .attr = Bar Attr
-term = Term
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_eq!(bundle.has_message("foo"), true);
    assert_eq!(bundle.has_message("-term"), false);
    assert_eq!(bundle.has_message("missing"), false);
    assert_eq!(bundle.has_message("-missing"), false);
}
