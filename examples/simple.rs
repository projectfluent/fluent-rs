extern crate fluent;

use fluent::context::MessageContext;

fn main() {
    let mut ctx = MessageContext::new();

    ctx.add_messages("key1 = Value 1");
    ctx.add_messages("key2 = Value 2");

    let msg = ctx.get_message("key1").unwrap();

    match ctx.format(&msg) {
        Ok(v) => println!("{}", v),
        Err(err) => println!("Couldn't retrieve: {:?}", err),
    }
}
