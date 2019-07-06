use std::borrow::Cow;

use fluent_bundle::bundle::FluentError;
use fluent_bundle::bundle::Message;
use fluent_bundle::{FluentBundle, FluentResource};

#[allow(dead_code)]
pub fn assert_format_none(result: Option<(Cow<str>, Vec<FluentError>)>) {
    assert!(result.is_none());
}

#[allow(dead_code)]
pub fn assert_format_no_errors(result: Option<(Cow<str>, Vec<FluentError>)>, expected: &str) {
    assert!(result.is_some());
    assert_eq!(result, Some((expected.into(), vec![])));
}

#[allow(dead_code)]
pub fn assert_format(
    result: Option<(Cow<str>, Vec<FluentError>)>,
    expected: &str,
    errors: Vec<FluentError>,
) {
    assert!(result.is_some());
    assert_eq!(result, Some((expected.into(), errors)));
}

#[allow(dead_code)]
pub fn assert_compound_no_errors(result: Option<(Message, Vec<FluentError>)>, expected: Message) {
    assert_eq!(result, Some((expected, vec![])));
}

#[allow(dead_code)]
pub fn assert_compound(
    result: Option<(Message, Vec<FluentError>)>,
    expected: Message,
    errors: Vec<FluentError>,
) {
    assert_eq!(result, Some((expected, errors)));
}

pub fn assert_get_resource_from_str_no_errors(s: &str) -> FluentResource {
    FluentResource::try_new(s.to_owned()).expect("Failed to parse an FTL resource.")
}

pub fn assert_get_bundle_no_errors<'a>(
    res: FluentResource,
    locale: Option<&str>,
) -> FluentBundle<'a> {
    let mut bundle = FluentBundle::new(&[locale.unwrap_or("x-testing")]);
    bundle
        .add_resource(res)
        .expect("Failed to add FluentResource to FluentBundle.");
    bundle
}