mod helpers;
use fluent_bundle::errors::FluentError;
use fluent_bundle::resolve::ResolverError;
use fluent_bundle::FluentValue;
use std::collections::HashMap;

use self::helpers::{
    assert_format, assert_format_no_errors, assert_format_none, assert_get_bundle_no_errors,
    assert_get_resource_from_str_no_errors,
};

#[test]
fn functions_missing() {
    let res = assert_get_resource_from_str_no_errors(
        r#"
foo = { MISSING("Foo") }
                                                     "#,
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_format(
        bundle.format("foo", None),
        "MISSING()",
        vec![FluentError::ResolverError(ResolverError::Reference(
            "Unknown function: MISSING()".into(),
        ))],
    );
}

#[test]
fn functions_arguments() {
    let res = assert_get_resource_from_str_no_errors(
        r#"
foo = Foo
    .attr = Attribute
pass-nothing       = { IDENTITY() }
pass-string        = { IDENTITY("a") }
pass-number        = { IDENTITY(1) }
pass-message       = { IDENTITY(foo) }
pass-attr          = { IDENTITY(foo.attr) }
pass-variable      = { IDENTITY($var) }
pass-function-call = { IDENTITY(IDENTITY(1)) }
    "#,
    );
    let mut bundle = assert_get_bundle_no_errors(&res, None);
    bundle
        .add_function("IDENTITY", |args, _named_args| {
            if let Some(arg) = args.get(0) {
                arg.clone().into()
            } else {
                FluentValue::None()
            }
        })
        .expect("Failed to add a function to the bundle.");

    // XXX: Different result from JS because
    // we don't handle argument errors out
    // of functions yet.
    assert_format_no_errors(bundle.format("pass-nothing", None), "???");

    assert_format_no_errors(bundle.format("pass-string", None), "a");

    assert_format_no_errors(bundle.format("pass-number", None), "1");

    assert_format_no_errors(bundle.format("pass-message", None), "Foo");

    assert_format_no_errors(bundle.format("pass-attr", None), "Attribute");

    let mut args = HashMap::new();
    args.insert("var", "Variable".into());
    assert_format_no_errors(bundle.format("pass-variable", Some(&args)), "Variable");

    assert_format_no_errors(bundle.format("pass-function-call", None), "1");
}
