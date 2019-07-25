use fluent_bundle::{FluentBundle, FluentResource, FluentValue};
use std::collections::HashMap;
use std::convert::TryFrom;
use unic_langid::LanguageIdentifier;

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
    let langid_en = LanguageIdentifier::try_from("en").expect("Parsing failed.");
    let mut bundle = FluentBundle::new(&[langid_en]);
    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    let mut args = HashMap::new();
    args.insert("name".to_string(), FluentValue::from("John"));

    let msg = bundle
        .get_message("hello-world")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value.expect("Message has no value.");
    let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);
    println!("{}", value);

    let msg = bundle.get_message("ref").expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value.expect("Message has no value.");
    let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);
    println!("{}", value);

    let mut args = HashMap::new();
    args.insert("emailCount".to_string(), FluentValue::into_number("1.0"));

    let msg = bundle
        .get_message("unread-emails")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value.expect("Message has no value.");
    let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);
    println!("{}", value);
}
