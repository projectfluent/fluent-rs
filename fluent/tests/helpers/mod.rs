use fluent::bundle::FluentError;
use fluent::bundle::Message;

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

pub fn assert_add_messages_no_errors(result: Result<(), Vec<FluentError>>) {
    assert!(result.is_ok());
}
