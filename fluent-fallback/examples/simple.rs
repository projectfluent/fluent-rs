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
use elsa::FrozenMap;
use fluent_bundle::{FluentBundle, FluentResource, FluentValue};
use fluent_fallback::Localization;
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
    let resources: FrozenMap<String, Box<FluentResource>> = FrozenMap::new();

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

    // 5. Construct a callback that will be used by the Localization instance to rebuild
    //    the iterator over FluentBundle instances.
    let res_path_scheme = "./examples/resources/{locale}/{res_id}";
    let generate_messages = |res_ids: &[String]| {
        let mut bundles = vec![];

        for locale in locales.clone() {
            let mut bundle = FluentBundle::new(&[&locale]);
            let res_path = res_path_scheme.replace("{locale}", &locale);
            for res_id in res_ids {
                let path = res_path.replace("{res_id}", res_id);
                let res = if let Some(res) = resources.get(&path) {
                    res
                } else {
                    let source = read_file(&path).unwrap();
                    let res = FluentResource::try_new(source).unwrap();
                    resources.insert(path, Box::new(res))
                };
                bundle.add_resource(&res).unwrap();
            }
            bundles.push(bundle);
        }

        return bundles.into_iter();
    };

    // 6. Create a new Localization instance which will be used to maintain the localization
    //    context for this UI.
    let mut loc = Localization::new(
        L10N_RESOURCES.iter().map(|s| s.to_string()).collect(),
        generate_messages,
    );

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
                    let value = loc.format_value("response-msg", Some(&args));
                    println!("{}", value);
                }
                Err(err) => {
                    let mut args = HashMap::new();
                    args.insert("input", FluentValue::from(input.as_str()));
                    args.insert("reason", FluentValue::from(err.to_string()));
                    let value = loc.format_value("input-parse-error-msg", Some(&args));
                    println!("{}", value);
                }
            }
        }
        None => {
            let value = loc.format_value("missing-arg-error", None);
            println!("{}", value);
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
