mod helpers;

use self::helpers::{
    assert_format_message_no_errors, assert_get_bundle_no_errors,
    assert_get_resource_from_str_no_errors,
};
use fluent_bundle::bundle::Message;
use std::collections::HashMap;

#[test]
fn format() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo = Foo
    .attr = Attribute
    .attr2 = Attribute 2
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, Some("en"));

    let mut attrs = HashMap::new();
    attrs.insert("attr".to_string(), "Attribute".to_string());
    attrs.insert("attr2".to_string(), "Attribute 2".to_string());

    assert_format_message_no_errors(
        bundle.format_message("foo", None),
        Message {
            value: Some("Foo".to_string()),
            attributes: attrs,
        },
    );
}
