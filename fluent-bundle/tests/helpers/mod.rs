use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::bundle::FluentError;
use fluent_bundle::bundle::Message;
use fluent_bundle::resource::FluentResource;

#[allow(dead_code)]
pub fn assert_format_none(result: Option<(String, Vec<FluentError>)>) {
    assert!(result.is_none());
}

#[allow(dead_code)]
pub fn assert_format_no_errors(result: Option<(String, Vec<FluentError>)>, expected: &str) {
    assert!(result.is_some());
    assert_eq!(result, Some((expected.to_string(), vec![])));
}

#[allow(dead_code)]
pub fn assert_format_message_no_errors(
    result: Option<(Message, Vec<FluentError>)>,
    expected: Message,
) {
    assert_eq!(result, Some((expected, vec![])));
}

pub fn assert_get_resource_from_str_no_errors(s: &str) -> FluentResource {
    FluentResource::try_new(s.to_owned()).unwrap()
}

pub fn assert_get_bundle_no_errors<'a>(
    res: &'a FluentResource,
    locale: Option<&str>,
) -> FluentBundle<'a> {
    let mut bundle = FluentBundle::new(&[locale.unwrap_or("x-testing")]);
    bundle
        .add_resource(res)
        .expect("Failed to add FluentResource to FluentBundle.");
    bundle
}
