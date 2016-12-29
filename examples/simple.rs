extern crate fluent;

use fluent::context::MessageContext;

fn main() {
    let mut ctx = MessageContext::new();

    ctx.add_messages("key1 = Value 1");
    ctx.add_messages("key2 = Value 2");

    let id = String::from("key1");

    match ctx.format(&id) {
        Ok(v) => println!("Message for key {} - {}", &id, v),
        Err(err) => println!("Couldn't retrieve the entry {:?}: {:?}", &id, err),
    }
}
