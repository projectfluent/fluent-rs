//! This is an example of a simple application
//! which calculates the Collatz conjecture.
//!
//! The function itself is trivial on purpose,
//! so that we can focus on understanding how
//! the application can be made localizable
//! via Fluent.
//!
//! To try the app launch `cargo run --example simple NUM (LOCALES)`
//!
//! NUM is a number to be calculated, and LOCALES is an optional
//! parameter with a comma-separated list of locales requested by the user.
//!
//! Example:
//!   
//!   caron run --example simple 123 de,pl
//!
//! If the second argument is omitted, `en-US` locale is used as the
//! default one.
use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::resource::FluentResource;
use fluent_bundle::types::FluentValue;
use fluent_locale::{negotiate_languages, NegotiationStrategy};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

/// We need a generic file read helper function to
/// read the localization resource file.
///
/// The resource files are stored in
/// `./examples/resources/{locale}` directory.
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

/// This helper function allows us to read the list
/// of available locales by reading the list of
/// directories in `./examples/resources`.
///
/// It is expected that every directory inside it
/// has a name that is a valid BCP47 language tag.
fn get_available_locales() -> Result<Vec<String>, io::Error> {
    let mut locales = vec![];

    let res_dir = fs::read_dir("./examples/resources/")?;
    for entry in res_dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if let Some(name) = name.to_str() {
                        locales.push(String::from(name));
                    }
                }
            }
        }
    }
    return Ok(locales);
}

/// This function negotiates the locales between available
/// and requested by the user.
///
/// It uses `fluent-locale` library but one could
/// use any other that will resolve the list of
/// available locales based on the list of
/// requested locales.
fn get_app_locales(requested: &[&str]) -> Result<Vec<String>, io::Error> {
    let available = get_available_locales()?;
    let resolved_locales = negotiate_languages(
        requested,
        &available,
        Some("en-US"),
        &NegotiationStrategy::Filtering,
    );
    return Ok(resolved_locales
        .into_iter()
        .map(|s| String::from(s))
        .collect::<Vec<String>>());
}

static L10N_RESOURCES: &[&str] = &["simple.ftl"];

fn main() {
    // 1. Get the command line arguments.
    let args: Vec<String> = env::args().collect();

    // 2. Allocate resources.
    let mut resources: Vec<FluentResource> = vec![];

    // 3. If the argument length is more than 1,
    //    take the second argument as a comma-separated
    //    list of requested locales.
    //
    //    Otherwise, take ["en-US"] as the default.
    let requested = args
        .get(2)
        .map_or(vec!["en-US"], |arg| arg.split(",").collect());

    // 4. Negotiate it against the avialable ones
    let locales = get_app_locales(&requested).expect("Failed to retrieve available locales");

    // 5. Create a new Fluent FluentBundle using the
    //    resolved locales.
    let mut bundle = FluentBundle::new(&locales);

    // 6. Load the localization resource
    for path in L10N_RESOURCES {
        let full_path = format!(
            "./examples/resources/{locale}/{path}",
            locale = locales[0],
            path = path
        );
        let source = read_file(&full_path).unwrap();
        let resource = FluentResource::try_new(source).unwrap();
        resources.push(resource);
    }

    for res in &resources {
        bundle.add_resource(res).unwrap();
    }

    // 7. Check if the input is provided.
    match args.get(1) {
        Some(input) => {
            // 7.1. Cast it to a number.
            match isize::from_str(&input) {
                Ok(i) => {
                    // 7.2. Construct a map of arguments
                    //      to format the message.
                    let mut args = HashMap::new();
                    args.insert("input", FluentValue::from(i));
                    args.insert("value", FluentValue::from(collatz(i)));
                    // 7.3. Format the message.
                    println!("{}", bundle.format("response-msg", Some(&args)).unwrap().0);
                }
                Err(err) => {
                    let mut args = HashMap::new();
                    args.insert("input", FluentValue::from(input.to_string()));
                    args.insert("reason", FluentValue::from(err.to_string()));
                    println!(
                        "{}",
                        bundle
                            .format("input-parse-error-msg", Some(&args))
                            .unwrap()
                            .0
                    );
                }
            }
        }
        None => {
            println!("{}", bundle.format("missing-arg-error", None).unwrap().0);
        }
    }
}

/// Collatz conjecture calculating function.
fn collatz(n: isize) -> isize {
    match n {
        1 => 0,
        _ => match n % 2 {
            0 => 1 + collatz(n / 2),
            _ => 1 + collatz(n * 3 + 1),
        },
    }
}
