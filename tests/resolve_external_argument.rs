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

    let value = ctx.get_message("hello-world").and_then(|msg| {
        ctx.format(msg, Some(&args))
    });

    assert_eq!(value, Some("Hello John".to_string()));
}

#[test]
fn external_argument_number() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages("unread-emails = You have { $emailsCount } unread emails.");

    let mut args = HashMap::new();
    args.insert("emailsCount", FluentValue::from(5));

    let value = ctx.get_message("unread-emails").and_then(|msg| {
        ctx.format(msg, Some(&args))
    });

    assert_eq!(value, Some("You have 5 unread emails.".to_string()));
}

#[test]
fn reference_message_with_external_argument() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages("greetings = Hello, { $userName }");
    ctx.add_messages("click-on = Click on the `{ greetings }` label.");

    let mut args = HashMap::new();
    args.insert("userName", FluentValue::from("Mary"));

    let value = ctx.get_message("click-on").and_then(|msg| {
        ctx.format(msg, Some(&args))
    });

    assert_eq!(value, Some("Click on the `Hello, Mary` label.".to_string()));
}
