extern crate fluent;

use fluent::MessageContext;
use fluent::types::FluentValue;

fn main() {
    let mut ctx = MessageContext::new(&["en-US"]);

    // Test for a simple function that returns a string
    ctx.add_function(
        "HELLO",
        Box::new(|_args, _named_args| {
            return Some("I'm a function!".to_string().into());
        }),
    );

    // Test for a function that accepts unnamed arguments
    ctx.add_function(
        "MEANING_OF_LIFE",
        Box::new(|args, _named_args| {
            if args.len() > 0 {
                if args[0] == FluentValue::Number(42.0) {
                    return Some(
                        "The answer to life, the universe, and everything"
                            .to_string()
                            .into(),
                    );
                }
            }

            None
        }),
    );

    // Test for a function that accepts named arguments
    ctx.add_function(
        "BASE_OWNERSHIP",
        Box::new(|_args, named_args| {
            let ownership = named_args.get("ownership").unwrap();

            return match ownership {
                FluentValue::String(string) => Some(
                    format!("All your base belong to {}", string)
                        .to_string()
                        .into(),
                ),
                _ => None,
            };
        }),
    );

    ctx.add_messages("hello-world = Hey there! { HELLO() }");
    ctx.add_messages("meaning-of-life = { MEANING_OF_LIFE(42) }");
    ctx.add_messages("all-your-base = { BASE_OWNERSHIP(hello, ownership: \"us\") }");

    let value = ctx.get_message("hello-world")
        .and_then(|message| ctx.format(message, None));
    assert_eq!(value, Some("Hey there! I'm a function!".to_string()));

    let value = ctx.get_message("meaning-of-life")
        .and_then(|message| ctx.format(message, None));
    assert_eq!(
        value,
        Some("The answer to life, the universe, and everything".to_string())
    );

    let value = ctx.get_message("all-your-base")
        .and_then(|message| ctx.format(message, None));
    assert_eq!(value, Some("All your base belong to us".to_string()));
}
