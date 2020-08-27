//! This is an example of a simple application
//! which calculates the Collatz conjecture.
//!
//! The function itself is trivial on purpose,
//! so that we can focus on understanding how
//! the application can be made localizable
//! via Fluent.
//!
//! To try the app launch `cargo run --example simple-fallback NUM (LOCALES)`
//!
//! NUM is a number to be calculated, and LOCALES is an optional
//! parameter with a comma-separated list of locales requested by the user.
//!
//! Example:
//!   
//!   cargo run --example simple-fallback 123 de,pl
//!
//! If the second argument is omitted, `en-US` locale is used as the
//! default one.
use elsa::FrozenMap;
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use fluent_fallback::Localization;
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

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
fn get_available_locales() -> Result<Vec<LanguageIdentifier>, io::Error> {
    let mut locales = vec![];

    let mut dir = env::current_dir()?;
    if dir.to_string_lossy().ends_with("fluent-rs") {
        dir.push("fluent-fallback");
    }
    dir.push("examples");
    dir.push("resources");
    let res_dir = fs::read_dir(dir)?;
    for entry in res_dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if let Some(name) = name.to_str() {
                        let langid: LanguageIdentifier = name.parse().expect("Parsing failed.");
                        locales.push(langid);
                    }
                }
            }
        }
    }
    return Ok(locales);
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
    let requested: Vec<LanguageIdentifier> = args.get(2).map_or(vec![], |arg| {
        arg.split(",")
            .map(|s| s.parse().expect("Parsing locale failed."))
            .collect()
    });

    // 4. Negotiate it against the avialable ones
    let default_locale: LanguageIdentifier = "en-US".parse().expect("Parsing failed.");
    let available = get_available_locales().expect("Retrieving available locales failed.");
    let resolved_locales = negotiate_languages(
        &requested,
        &available,
        Some(&default_locale),
        NegotiationStrategy::Filtering,
    );

    // 5. Construct a callback that will be used by the Localization instance to rebuild
    //    the iterator over FluentBundle instances.
    // let res_path_scheme = "./examples/resources/{locale}/{res_id}";
    let mut res_path_scheme = env::current_dir().expect("Failed to retireve current dir.");
    if res_path_scheme.to_string_lossy().ends_with("fluent-rs") {
        res_path_scheme.push("fluent-bundle");
    }
    res_path_scheme.push("examples");
    res_path_scheme.push("resources");
    res_path_scheme.push("{locale}");
    res_path_scheme.push("{res_id}");
    let res_path_scheme = res_path_scheme.to_str().unwrap();
    let generate_messages = |res_ids: Vec<PathBuf>| {
        let mut locales = resolved_locales.iter();
        let res_mgr = &resources;
        let res_ids = res_ids.to_vec();

        iter::from_fn(move || {
            locales.next().map(|locale| {
                let mut bundle = FluentBundle::new(vec![locale.clone()]);
                let res_path = res_path_scheme.replace("{locale}", &locale.to_string());

                for res_id in &res_ids {
                    let path = res_path.replace("{res_id}", res_id.to_str().unwrap());
                    let res = res_mgr.get(&path).unwrap_or_else(|| {
                        let source = read_file(&path).unwrap();
                        let res = FluentResource::try_new(source).unwrap();
                        res_mgr.insert(path.to_string(), Box::new(res))
                    });
                    bundle.add_resource(res).unwrap();
                }
                bundle
            })
        })
    };

    // 6. Create a new Localization instance which will be used to maintain the localization
    //    context for this UI.
    let loc = Localization::new(
        L10N_RESOURCES.iter().map(|res| res.into()).collect(),
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
                    let mut args = FluentArgs::new();
                    args.add("input", FluentValue::from(i));
                    args.add("value", FluentValue::from(collatz(i)));
                    // 7.3. Format the message.
                    let value = loc.format_value("response-msg", Some(&args));
                    println!("{}", value);
                }
                Err(err) => {
                    let mut args = FluentArgs::new();
                    args.add("input", FluentValue::from(input.as_str()));
                    args.add("reason", FluentValue::from(err.to_string()));
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
