extern crate fluent;

use fluent::context::MessageContext;

fn main() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages(
        "
foo = Foo
foobar = { foo } Bar
bazbar = { baz } Bar
",
    );

    match ctx
        .get_message("foobar")
        .and_then(|msg| ctx.format(msg, None))
    {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    match ctx
        .get_message("bazbar")
        .and_then(|msg| ctx.format(msg, None))
    {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }
}
