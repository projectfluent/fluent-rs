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

    match ctx.format("foobar", None) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    match ctx.format("bazbar", None) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }
}
