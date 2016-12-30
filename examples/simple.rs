extern crate fluent;

use fluent::context::MessageContext;

#[allow(unused_must_use)]
fn main() {
    let mut ctx = MessageContext::new();

    ctx.add_messages("key1 = Value 1");
    ctx.add_messages("key2 = Value 2");

    if let Some(msg) = ctx.get_message("key1") {
        match ctx.format(&msg) {
            Ok(v) => println!("Formatted value: {}", v),
            Err(err) => println!("Formatting error: {:?}", err),
        }
    } else {
        println!("Missing message: {}", "key1");
    }

}
