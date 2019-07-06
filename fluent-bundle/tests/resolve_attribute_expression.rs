mod helpers;
use fluent_bundle::errors::FluentError;
use fluent_bundle::resolve::ResolverError;

use self::helpers::{
    assert_format, assert_format_no_errors, assert_get_bundle_no_errors,
    assert_get_resource_from_str_no_errors,
};

#[test]
fn attribute_expression() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo = Foo
    .attr = Foo Attr
bar =
    .attr = Bar Attr

use-foo = { foo }
use-foo-attr = { foo.attr }
use-bar = { bar }
use-bar-attr = { bar.attr }

missing-attr = { foo.missing }
missing-missing = { missing.missing }
    ",
    );
    let bundle = assert_get_bundle_no_errors(res, None);

    assert_format_no_errors(bundle.format("use-foo", None), "Foo");

    assert_format_no_errors(bundle.format("use-foo-attr", None), "Foo Attr");

    assert_format(
        bundle.format("use-bar", None),
        "bar",
        vec![FluentError::ResolverError(ResolverError::Reference(
            "Unknown message: bar".into(),
        ))],
    );

    assert_format_no_errors(bundle.format("use-bar-attr", None), "Bar Attr");

    assert_format(
        bundle.format("missing-attr", None),
        "foo.missing",
        vec![FluentError::ResolverError(ResolverError::Reference(
            "Unknown message: foo.missing".into(),
        ))],
    );

    assert_format(
        bundle.format("missing-missing", None),
        "missing.missing",
        vec![FluentError::ResolverError(ResolverError::Reference(
            "Unknown message: missing.missing".into(),
        ))],
    );
}

#[test]
fn attribute_reference_cyclic() {
    {
        let res = assert_get_resource_from_str_no_errors(
            "
foo =
  .label = Foo { foo.label2 }
  .label2 = { foo.label3 } Baz
  .label3 = { foo.label } Bar
        ",
        );
        let bundle = assert_get_bundle_no_errors(res, None);

        assert_format(
            bundle.format("foo.label", None),
            "Foo foo.label Bar Baz",
            vec![FluentError::ResolverError(ResolverError::Cyclic)],
        );
    }

    {
        let res = assert_get_resource_from_str_no_errors(
            "
foo =
  .label = Foo { bar.label }
bar =
  .label = Bar { baz.label }
baz =
  .label = Baz
",
        );
        let bundle = assert_get_bundle_no_errors(res, None);

        assert_format_no_errors(bundle.format("foo.label", None), "Foo Bar Baz");
    }
}
