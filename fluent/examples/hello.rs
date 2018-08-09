extern crate fluent;

use fluent::MessageContext;

fn main() {
    let mut ctx = MessageContext::new(&["en-US"]);
    ctx.add_messages("hello-world = Hello, world!");

    let value = ctx.format("hello-world", None);
    assert_eq!(value, Some("Hello, world!".to_string()));
}
