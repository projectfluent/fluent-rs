use fluent_bundle::{FluentBundle, FluentResource, FluentValue};
use std::collections::HashMap;

fn main() {
    let res = FluentResource::try_new(
        "
hello-world = Hello {
       *[one] World
        [two] Moon
    }

hello-world2 = Hello { $name ->
       *[world] World
        [moon] Moon
    }
    "
        .to_owned(),
    )
    .unwrap();
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_resource(&res).unwrap();

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
