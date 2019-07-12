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
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    let msg = bundle
        .get_message("hello-world")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value.expect("Message has no value.");
    let value = bundle.format_pattern(&pattern, None, &mut errors);
    println!("{}", value);

    let mut args = HashMap::new();
    args.insert("name".to_string(), FluentValue::from("moon"));

    let msg = bundle
        .get_message("hello-world2")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value.expect("Message has no value.");
    let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);
    println!("{}", value);
}
