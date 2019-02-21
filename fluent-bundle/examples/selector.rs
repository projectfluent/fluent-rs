use fluent_bundle::{FluentBundle, FluentResource, FluentValue};
use std::collections::HashMap;

fn main() {
    let ftl_string = String::from(
        "
hello-world = Hello { $missing ->
       *[one] World
        [two] Moon
    }

hello-world2 = Hello { $name ->
       *[world] World
        [moon] Moon
    }
    ",
    );
    let res = FluentResource::try_new(ftl_string).expect("Could not parse an FTL string.");
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle
        .add_resource(&res)
        .expect("Failed to add FTL resources to the bundle.");

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
