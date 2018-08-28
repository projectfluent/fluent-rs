extern crate fluent;

use fluent::FluentBundle;

fn main() {
    let mut bundle = FluentBundle::new(&["en-US"]);
    bundle.add_messages(
        "
hello-world = Hello, world!
    .title = Foo
",
    );

    let value = bundle.format_message("hello-world", None);
    println!("{:#?}", value);
    // assert_eq!(value, Some("Hello, world!".to_string()));
}
