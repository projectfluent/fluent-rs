mod helpers;

use std::collections::HashMap;

use self::helpers::{
    assert_format_no_errors, assert_get_bundle_no_errors, assert_get_resource_from_str_no_errors,
};
use fluent_bundle::types::FluentValue;

#[test]
fn external_argument_string() {
    let res = assert_get_resource_from_str_no_errors("hello-world = Hello { $name }");
    let bundle = assert_get_bundle_no_errors(&res, None);

    let mut args = HashMap::new();
    args.insert("name", FluentValue::from("John"));

    assert_format_no_errors(bundle.format("hello-world", Some(&args)), "Hello John");
}

#[test]
fn external_argument_number() {
    let res = assert_get_resource_from_str_no_errors(
        "
unread-emails = You have { $emailsCount } unread emails.
unread-emails-dec = You have { $emailsCountDec } unread emails.
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    let mut args = HashMap::new();
    args.insert("emailsCount", FluentValue::from(5));
    args.insert("emailsCountDec", FluentValue::as_number("5.0").unwrap());

    assert_format_no_errors(
        bundle.format("unread-emails", Some(&args)),
        "You have 5 unread emails.",
    );

    assert_format_no_errors(
        bundle.format("unread-emails-dec", Some(&args)),
        "You have 5.0 unread emails.",
    );
}

#[test]
fn reference_message_with_external_argument() {
    let res = assert_get_resource_from_str_no_errors(
        "
greetings = Hello, { $userName }
click-on = Click on the `{ greetings }` label.
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    let mut args = HashMap::new();
    args.insert("userName", FluentValue::from("Mary"));

    assert_format_no_errors(
        bundle.format("click-on", Some(&args)),
        "Click on the `Hello, Mary` label.",
    );
}
