extern crate fluent;

use std::collections::HashMap;

use self::fluent::context::MessageContext;
use self::fluent::types::FluentValue;

#[test]
fn external_argument_string() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages("hello-world = Hello { $name }");

    let mut args = HashMap::new();
    args.insert("name", FluentValue::from("John"));

    let value = ctx.format("hello-world", Some(&args));
    assert_eq!(value, Some("Hello John".to_string()));
}

#[test]
fn external_argument_number() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages("unread-emails = You have { $emailsCount } unread emails.");
    ctx.add_messages("unread-emails-dec = You have { $emailsCountDec } unread emails.");

    let mut args = HashMap::new();
    args.insert("emailsCount", FluentValue::from(5));
    args.insert("emailsCountDec", FluentValue::as_number("5.0").unwrap());

    let value = ctx.format("unread-emails", Some(&args));
    assert_eq!(value, Some("You have 5 unread emails.".to_string()));

    let value = ctx.format("unread-emails-dec", Some(&args));
    assert_eq!(value, Some("You have 5.0 unread emails.".to_string()));
}

#[test]
fn reference_message_with_external_argument() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages("greetings = Hello, { $userName }");
    ctx.add_messages("click-on = Click on the `{ greetings }` label.");

    let mut args = HashMap::new();
    args.insert("userName", FluentValue::from("Mary"));

    let value = ctx.format("click-on", Some(&args));
    assert_eq!(value, Some("Click on the `Hello, Mary` label.".to_string()));
}
