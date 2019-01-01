use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::types::FluentValue;
use std::collections::HashMap;

fn main() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle
        .add_messages(
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
        )
        .unwrap();

    match bundle.format("hello-world", None) {
        Some((value, _)) => println!("{}", value),
        _ => println!("None"),
    }

    let mut args = HashMap::new();
    args.insert("name", FluentValue::from("moon"));

    match bundle.format("hello-world2", Some(&args)) {
        Some((value, _)) => println!("{}", value),
        _ => println!("None"),
    }
}
