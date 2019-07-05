mod helpers;
use fluent_bundle::errors::FluentError;
use fluent_bundle::resolve::ResolverError;

use self::helpers::{
    assert_compound, assert_compound_no_errors, assert_get_bundle_no_errors,
    assert_get_resource_from_str_no_errors,
    assert_get_resource_from_str_no_errors_rc,
    assert_get_bundle_no_errors_rc,
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
    attrs.insert("attr".into(), "Attribute".into());
    attrs.insert("attr2".into(), "Attribute 2".into());

    assert_compound_no_errors(
        bundle.compound("foo", None),
        Message {
            value: Some("Foo".into()),
            attributes: attrs,
        },
    );
}

#[test]
fn message_reference_cyclic() {
    {
        let res = assert_get_resource_from_str_no_errors_rc(
            "
foo = Foo { bar }
bar = { foo } Bar
        ",
        );
        let bundle = assert_get_bundle_no_errors_rc(res, None);

        assert_compound(
            bundle.compound("foo", None),
            Message {
                value: Some("Foo foo Bar".into()),
                attributes: HashMap::new(),
            },
            vec![FluentError::ResolverError(ResolverError::Cyclic)],
        );
    }
}
