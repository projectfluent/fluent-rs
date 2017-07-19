extern crate fluent;

use self::fluent::context::MessageContext;
use self::fluent::context::FluentArgument;
use std::collections::HashMap;

#[test]
fn external_argument_string() {
    let locales = vec![String::from("pl")];
    let mut ctx = MessageContext::new(locales);

    ctx.add_messages("hello-world = Hello { $name }");

    let mut args = HashMap::new();
    args.insert("name", FluentArgument::from("John"));

    let val = match ctx.get_message("hello-world")
              .and_then(|msg| ctx.format(&msg, Some(&args))) {
        Some(value) => value,
        None => String::from("None"),
    };

    assert_eq!(String::from("Hello John"), val);
}

#[test]
fn external_argument_number() {
    let locales = vec![String::from("pl")];
    let mut ctx = MessageContext::new(locales);

    ctx.add_messages("unread-emails = You have { $emailsCount } unread emails");

    let mut args = HashMap::new();
    args.insert("emailsCount", FluentArgument::from(5));

    let val = match ctx.get_message("unread-emails")
              .and_then(|msg| ctx.format(&msg, Some(&args))) {
        Some(value) => value,
        None => String::from("None"),
    };

    assert_eq!(String::from("You have 5 unread emails"), val);
}

#[test]
fn reference_message_with_external_argument() {
    let locales = vec![String::from("pl")];
    let mut ctx = MessageContext::new(locales);

    ctx.add_messages("greetings = Hello, { $userName }");
    ctx.add_messages("click-on = Click on the `{ greetings }` label.");

    let mut args = HashMap::new();
    args.insert("userName", FluentArgument::from("Mary"));

    let val = match ctx.get_message("click-on")
              .and_then(|msg| ctx.format(&msg, Some(&args))) {
        Some(value) => value,
        None => String::from("None"),
    };

    assert_eq!(String::from("Click on the `Hello, Mary` label."), val);
}
