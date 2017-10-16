extern crate fluent;

use fluent::context::MessageContext;
use fluent::types::FluentValue;
use std::collections::HashMap;

fn main() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages(
        "
hello-world = Hello {
       *[one] World
        [two] Moon
    }

hello-world2 = Hello { $name ->
       *[world] World
        [moon] Moon
    }
",
    );

    match ctx.get_message("hello-world")
        .and_then(|msg| ctx.format(msg, None))
    {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    let mut args = HashMap::new();
    args.insert("name", FluentValue::from("moon"));

    match ctx.get_message("hello-world2")
        .and_then(|msg| ctx.format(msg, Some(&args)))
    {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }
}
