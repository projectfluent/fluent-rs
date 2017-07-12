extern crate fluent;

use self::fluent::context::MessageContext;
use self::fluent::context::FluentArgument;
use std::collections::HashMap;

#[test]
fn external_argument() {
    let locales = vec![String::from("pl")];
    let mut ctx = MessageContext::new(locales);

    ctx.add_messages("hello-world = Hello { $name }");

    let mut args = HashMap::new();
    args.insert("name", FluentArgument::from("John"));

    let val = match ctx.get_message("hello-world")
              .and_then(|msg| ctx.format(&msg, Some(&args))) {
        Some(value) => value,
        None => String::from("None"),
    };

    assert_eq!(String::from("Hello John"), val);
}
