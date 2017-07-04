extern crate fluent;

use fluent::context::MessageContext;
use std::collections::HashMap;

fn main() {
    let locales = vec![String::from("pl")];
    let mut ctx = MessageContext::new(locales);

    ctx.add_messages("hello-world = Hello { $name }");

    let mut args = HashMap::new();
    args.insert(String::from("name"), String::from("John"));

    match ctx.get_message("hello-world")
              .and_then(|msg| ctx.format(&msg, Some(&args))) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }
}
