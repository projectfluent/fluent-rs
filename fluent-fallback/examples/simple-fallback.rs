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

use std::{env, fs, io, path::PathBuf, str::FromStr};

use fluent_bundle::{FluentArgs, FluentBundle, FluentError, FluentResource, FluentValue};
use fluent_fallback::{BundleGeneratorSync, SyncLocalization};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};

use unic_langid::LanguageIdentifier;

/// This helper struct holds the available locales and scheme for converting
/// resource paths into full paths. It is used to customise
/// `fluent-fallback::SyncLocalization`.
struct Bundles {
    locales: Vec<LanguageIdentifier>,
    res_path_scheme: PathBuf,
}

/// This helper function allows us to read the list
/// of available locales by reading the list of
/// directories in `./examples/resources`.
///
/// It is expected that every directory inside it
/// has a name that is a valid BCP47 language tag.
fn get_available_locales() -> io::Result<Vec<LanguageIdentifier>> {
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
    Ok(locales)
}

static L10N_RESOURCES: &[&str] = &["simple.ftl"];

fn main() {
    // 1. Get the command line arguments.
    let args: Vec<String> = env::args().collect();

    // 2. If the argument length is more than 1,
    //    take the second argument as a comma-separated
    //    list of requested locales.
    let requested: Vec<LanguageIdentifier> = args.get(2).map_or(vec![], |arg| {
        arg.split(",")
            .map(|s| s.parse().expect("Parsing locale failed."))
            .collect()
    });

    // 3. Negotiate it against the avialable ones
    let default_locale: LanguageIdentifier = "en-US".parse().expect("Parsing failed.");
    let available = get_available_locales().expect("Retrieving available locales failed.");
    let resolved_locales = negotiate_languages(
        &requested,
        &available,
        Some(&default_locale),
        NegotiationStrategy::Filtering,
    );

    // 4. Construct the path scheme for converting `locale` and `res_id` resource
    //    path into full path passed to OS for loading.
    //    Eg. ./examples/resources/{locale}/{res_id}
    let mut res_path_scheme = env::current_dir().expect("Failed to retrieve current dir.");
    if res_path_scheme.to_string_lossy().ends_with("fluent-rs") {
        res_path_scheme.push("fluent-bundle");
    }
    res_path_scheme.push("examples");
    res_path_scheme.push("resources");

    res_path_scheme.push("{locale}");
    res_path_scheme.push("{res_id}");

    // 5. Create a new Localization instance which will be used to maintain the localization
    //    context for this UI.  `Bundles` provides the custom logic for obtaining resources.
    let loc = SyncLocalization::with_generator(
        L10N_RESOURCES.iter().map(|&res| res.into()).collect(),
        Bundles {
            locales: resolved_locales.into_iter().cloned().collect(),
            res_path_scheme,
        },
    );

    // 6. Check if the input is provided.
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
                    let value = loc.format_value_sync("response-msg", Some(&args));
                    println!("{}", value);
                }
                Err(err) => {
                    let mut args = FluentArgs::new();
                    args.add("input", FluentValue::from(input.as_str()));
                    args.add("reason", FluentValue::from(err.to_string()));
                    let value = loc.format_value_sync("input-parse-error-msg", Some(&args));
                    println!("{}", value);
                }
            }
        }
        None => {
            let value = loc.format_value_sync("missing-arg-error", None);
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

/// Bundle iterator used by BundleGeneratorSync implementation for Locales.
struct BundleIter {
    res_path_scheme: String,
    locales: <Vec<LanguageIdentifier> as IntoIterator>::IntoIter,
    resource_ids: Vec<String>,
}

impl Iterator for BundleIter {
    type Item =
        Result<FluentBundle<FluentResource>, (FluentBundle<FluentResource>, Vec<FluentError>)>;

    fn next(&mut self) -> Option<Self::Item> {
        let locale = self.locales.next()?;
        let res_path_scheme = self
            .res_path_scheme
            .as_str()
            .replace("{locale}", &locale.to_string());
        let mut bundle = FluentBundle::new(vec![locale]);

        let mut errors = vec![];

        for res_id in &self.resource_ids {
            let res_path = res_path_scheme.as_str().replace("{res_id}", res_id);
            let source = fs::read_to_string(res_path).unwrap();
            let res = match FluentResource::try_new(source) {
                Ok(res) => res,
                Err((res, err)) => {
                    errors.extend(err.into_iter().map(Into::into));
                    res
                }
            };
            bundle.add_resource(res).unwrap();
        }

        if errors.is_empty() {
            Some(Ok(bundle))
        } else {
            Some(Err((bundle, errors)))
        }
    }
}

impl BundleGeneratorSync for Bundles {
    type Resource = FluentResource;
    type Iter = BundleIter;

    fn bundles_sync(&self, resource_ids: Vec<String>) -> Self::Iter {
        BundleIter {
            res_path_scheme: self.res_path_scheme.to_string_lossy().to_string(),
            locales: self.locales.clone().into_iter(),
            resource_ids,
        }
    }
}
