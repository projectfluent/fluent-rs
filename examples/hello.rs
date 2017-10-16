extern crate fluent;

use fluent::MessageContext;

fn main() {
    let mut ctx = MessageContext::new(&["en-US"]);
    ctx.add_messages("hello-world = Hello, world!");

    let value = ctx.get_message("hello-world").and_then(|message| {
        ctx.format(message, None)
    });

    assert_eq!(value, Some("Hello, world!".to_string()));
}
