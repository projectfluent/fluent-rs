mod helpers;
use fluent_bundle::errors::FluentError;
use fluent_bundle::resolve::ResolverError;

use self::helpers::{
    assert_format, assert_format_no_errors, assert_get_bundle_no_errors,
    assert_get_resource_from_str_no_errors,
    assert_get_resource_from_str_no_errors_rc, assert_get_bundle_no_errors_rc,
};

#[test]
fn message_reference() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo = Foo
bar = { foo } Bar
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_format_no_errors(bundle.format("bar", None), "Foo Bar");
}

#[test]
fn term_reference() {
    let res = assert_get_resource_from_str_no_errors(
        "
-foo = Foo
bar = { -foo } Bar
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_format_no_errors(bundle.format("bar", None), "Foo Bar");
}

#[test]
fn message_reference_nested() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo = Foo
bar = { foo } Bar
baz = { bar } Baz
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_format_no_errors(bundle.format("baz", None), "Foo Bar Baz");
}

#[test]
fn message_reference_missing() {
    let res = assert_get_resource_from_str_no_errors("bar = { foo } Bar");
    let bundle = assert_get_bundle_no_errors(&res, None);
    assert_format(
        bundle.format("bar", None),
        "foo Bar",
        vec![FluentError::ResolverError(ResolverError::Reference(
            "Unknown message: foo".into(),
        ))],
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

        assert_format(
            bundle.format("foo", None),
            "Foo foo Bar",
            vec![FluentError::ResolverError(ResolverError::Cyclic)],
        );
    }

    {
        let res = assert_get_resource_from_str_no_errors_rc(
            "
foo = { bar }
bar = { foo }
        ",
        );
        let bundle = assert_get_bundle_no_errors_rc(res, None);

        assert_format(
            bundle.format("foo", None),
            "foo",
            vec![FluentError::ResolverError(ResolverError::Cyclic)],
        );
        assert_format(
            bundle.format("bar", None),
            "bar",
            vec![FluentError::ResolverError(ResolverError::Cyclic)],
        );
    }
}

#[test]
fn message_reference_multiple() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo = Foo
bar = { foo } Bar { foo }
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_format_no_errors(bundle.format("bar", None), "Foo Bar Foo");
}
