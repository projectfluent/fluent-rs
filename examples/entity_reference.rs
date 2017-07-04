extern crate fluent;

use fluent::context::MessageContext;

fn main() {
    let locales = vec![String::from("pl"), String::from("en-US")];
    let mut ctx = MessageContext::new(locales);

    ctx.add_messages("foo = Foo");
    ctx.add_messages("foobar = { foo } Bar");
    ctx.add_messages("bazbar = { baz } Bar");

    match ctx.get_message("foobar")
              .and_then(|msg| ctx.format(&msg, None)) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    match ctx.get_message("bazbar")
              .and_then(|msg| ctx.format(&msg, None)) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }
}
