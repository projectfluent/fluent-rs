use fluent::context::FluentError;
use fluent::context::Message;

#[allow(dead_code)]
pub fn assert_format_none(result: Option<Result<String, (String, Vec<FluentError>)>>) {
    assert!(result.is_none());
}

#[allow(dead_code)]
pub fn assert_format_no_errors(
    result: Option<Result<String, (String, Vec<FluentError>)>>,
    expected: &str,
) {
    assert!(result.is_some());
    assert_eq!(result, Some(Ok(expected.to_string())));
}

#[allow(dead_code)]
pub fn assert_format_message_no_errors(
    result: Option<Result<Message, (Message, Vec<FluentError>)>>,
    expected: Message,
) {
    assert_eq!(result, Some(Ok(expected)));
}

pub fn assert_add_messages_no_errors(result: Result<(), Vec<FluentError>>) {
    assert!(result.is_ok());
}
