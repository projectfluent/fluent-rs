mod helpers;

use self::helpers::{
    assert_format_no_errors, assert_get_bundle_no_errors, assert_get_resource_from_str_no_errors,
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
    assert_format_no_errors(bundle.format("bar", None), "___ Bar");
}

#[test]
fn message_reference_cyclic() {
    {
        let res = assert_get_resource_from_str_no_errors(
            "
foo = Foo { bar }
bar = { foo } Bar
        ",
        );
        let bundle = assert_get_bundle_no_errors(&res, None);

        assert_format_no_errors(bundle.format("foo", None), "Foo ___");
        assert_format_no_errors(bundle.format("bar", None), "___ Bar");
    }

    {
        let res = assert_get_resource_from_str_no_errors(
            "
foo = { bar }
bar = { foo }
        ",
        );
        let bundle = assert_get_bundle_no_errors(&res, None);

        assert_format_no_errors(bundle.format("foo", None), "___");
        assert_format_no_errors(bundle.format("bar", None), "___");
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
