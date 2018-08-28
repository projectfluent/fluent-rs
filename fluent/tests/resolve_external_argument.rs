extern crate fluent;

mod helpers;

use std::collections::HashMap;

use self::fluent::context::FluentBundle;
use self::fluent::types::FluentValue;
use helpers::{assert_add_messages_no_errors, assert_format_no_errors};

#[test]
fn external_argument_string() {
    let mut bundle = FluentBundle::new(&["x-testing"]);

    assert_add_messages_no_errors(bundle.add_messages("hello-world = Hello { $name }"));

    let mut args = HashMap::new();
    args.insert("name", FluentValue::from("John"));

    assert_format_no_errors(bundle.format("hello-world", Some(&args)), "Hello John");
}

#[test]
fn external_argument_number() {
    let mut bundle = FluentBundle::new(&["x-testing"]);

    assert_add_messages_no_errors(
        bundle.add_messages("unread-emails = You have { $emailsCount } unread emails."),
    );
    assert_add_messages_no_errors(
        bundle.add_messages("unread-emails-dec = You have { $emailsCountDec } unread emails."),
    );

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
    let mut bundle = FluentBundle::new(&["x-testing"]);

    assert_add_messages_no_errors(bundle.add_messages("greetings = Hello, { $userName }"));
    assert_add_messages_no_errors(
        bundle.add_messages("click-on = Click on the `{ greetings }` label."),
    );

    let mut args = HashMap::new();
    args.insert("userName", FluentValue::from("Mary"));

    assert_format_no_errors(
        bundle.format("click-on", Some(&args)),
        "Click on the `Hello, Mary` label.",
    );
}
