use fluent_bundle::{FluentBundle, FluentResource, FluentValue};
use std::collections::HashMap;

fn main() {
    let ftl_string = String::from(
        "
hello-world = Hello { $name }
ref = The previous message says { hello-world }
unread-emails =
    { $emailCount ->
        [one] You have { $emailCount } unread email
       *[other] You have { $emailCount } unread emails
    }
    ",
    );
    let res = FluentResource::try_new(ftl_string).expect("Could not parse an FTL string.");
    let mut bundle = FluentBundle::new(&["en"]);
    bundle
        .add_resource(&res)
        .expect("Failed to add FTL resources to the bundle.");

    let mut args = HashMap::new();
    args.insert("name", FluentValue::from("John"));

    match bundle.format("hello-world", Some(&args)) {
        Some((value, _)) => println!("{}", value),
        _ => println!("None"),
    }

    match bundle.format("ref", Some(&args)) {
        Some((value, _)) => println!("{}", value),
        _ => println!("None"),
    }

    let mut args = HashMap::new();
    args.insert("emailCount", FluentValue::as_number("1.0").unwrap());

    match bundle.format("unread-emails", Some(&args)) {
        Some((value, _)) => println!("{}", value),
        None => println!("None"),
    }
}
