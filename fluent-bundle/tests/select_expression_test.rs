mod helpers;
use fluent_bundle::errors::FluentError;
use fluent_bundle::resolve::ResolverError;
use fluent_bundle::types::FluentValue;

use std::collections::HashMap;

use self::helpers::{
    assert_format, assert_format_no_errors, assert_get_bundle_no_errors,
    assert_get_resource_from_str_no_errors,
};

#[test]
fn missing_selector() {
    let res = assert_get_resource_from_str_no_errors(
        "
select = {$none ->
  [a] A
 *[b] B
}
    ",
    );
    let bundle = assert_get_bundle_no_errors(res, None);

    assert_format(
        bundle.format("select", None),
        "B",
        vec![FluentError::ResolverError(ResolverError::Reference(
            "Unknown variable: $none".into(),
        ))],
    );
}

#[test]
fn string_selector() {
    let res = assert_get_resource_from_str_no_errors(
        "
select = {$selector ->
  [a] A
 *[b] B
}
    ",
    );
    let bundle = assert_get_bundle_no_errors(res, None);

    let mut args = HashMap::new();
    args.insert("selector", FluentValue::from("a"));
    assert_format_no_errors(bundle.format("select", Some(&args)), "A");

    let mut args = HashMap::new();
    args.insert("selector", FluentValue::from("c"));
    assert_format_no_errors(bundle.format("select", Some(&args)), "B");
}

#[test]
fn number_selectors() {
    let res = assert_get_resource_from_str_no_errors(
        "
select = {$selector ->
  [0] A
 *[1] B
}
    ",
    );
    let bundle = assert_get_bundle_no_errors(res, None);

    let mut args = HashMap::new();
    args.insert("selector", FluentValue::from(0));
    assert_format_no_errors(bundle.format("select", Some(&args)), "A");

    let mut args = HashMap::new();
    args.insert("selector", FluentValue::from(2));
    assert_format_no_errors(bundle.format("select", Some(&args)), "B");
}

#[test]
fn plural_categories() {
    let res = assert_get_resource_from_str_no_errors(
        "
select = {$selector ->
  [one] A
 *[other] B
}
    ",
    );
    let bundle = assert_get_bundle_no_errors(res, None);

    let mut args = HashMap::new();
    args.insert("selector", FluentValue::from(1));
    assert_format_no_errors(bundle.format("select", Some(&args)), "A");

    let mut args = HashMap::new();
    args.insert("selector", FluentValue::from("one"));
    assert_format_no_errors(bundle.format("select", Some(&args)), "A");

    let res = assert_get_resource_from_str_no_errors(
        "
select = {$selector ->
  [one] A
 *[default] D
}
    ",
    );
    let bundle = assert_get_bundle_no_errors(res, None);

    let mut args = HashMap::new();
    args.insert("selector", FluentValue::from(2));
    assert_format_no_errors(bundle.format("select", Some(&args)), "D");

    let mut args = HashMap::new();
    args.insert("selector", FluentValue::from("default"));
    assert_format_no_errors(bundle.format("select", Some(&args)), "D");
}
