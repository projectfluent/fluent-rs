extern crate fluent;

use fluent::context::MessageContext;

fn main() {
    let mut ctx = MessageContext::new("und");

    ctx.add_messages("key1 = Value 1");
    ctx.add_messages("key2 = Value 2");

    match ctx.get_message("key1")
              .and_then(|msg| ctx.format(msg, None)) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    match ctx.get_message("key2")
              .and_then(|msg| ctx.format(msg, None)) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }
}
