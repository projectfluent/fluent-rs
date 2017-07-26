extern crate fluent;

use fluent::context::MessageContext;
use fluent::context::FluentArgument;
use std::collections::HashMap;

fn main() {
    let mut ctx = MessageContext::new("pl");

    ctx.add_messages("hello-world = Hello { $name }");
    ctx.add_messages("ref = The previous message says { hello-world }");
    ctx.add_messages("unread-emails = You have { $emailCount } unread emails");

    let mut args = HashMap::new();
    args.insert("name", FluentArgument::from("John"));

    match ctx.get_message("hello-world")
              .and_then(|msg| ctx.format(&msg, Some(&args))) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    match ctx.get_message("ref")
              .and_then(|msg| ctx.format(&msg, Some(&args))) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    let mut args = HashMap::new();
    args.insert("emailCount", FluentArgument::from(5));

    match ctx.get_message("unread-emails")
              .and_then(|msg| ctx.format(&msg, Some(&args))) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }
}
