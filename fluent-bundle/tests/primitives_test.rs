mod helpers;

use self::helpers::{
    assert_format_no_errors, assert_get_bundle_no_errors, assert_get_resource_from_str_no_errors,
};

#[test]
fn primitives_numbers() {
    let res = assert_get_resource_from_str_no_errors(
        "
one     = { 1 }
select  = { 1 ->
   *[0] Zero
    [1] One
}
    ",
    );
    let bundle = assert_get_bundle_no_errors(res, None);

    assert_format_no_errors(bundle.format("one", None), "1");

    assert_format_no_errors(bundle.format("select", None), "One");
}

#[test]
fn primitives_simple_string() {
    let res = assert_get_resource_from_str_no_errors(
        r#"
foo               = Foo

placeable-literal = { "Foo" } Bar
placeable-message = { foo } Bar

selector-literal = { "Foo" ->
   *[Foo] Member 1
}

bar =
    .attr = Bar Attribute

placeable-attr   = { bar.attr }

-baz = Baz
    .attr = BazAttribute

selector-attr    = { -baz.attr ->
   *[BazAttribute] Member 3
}
    "#,
    );
    let bundle = assert_get_bundle_no_errors(res, None);

    assert_format_no_errors(bundle.format("foo", None), "Foo");

    assert_format_no_errors(bundle.format("placeable-literal", None), "Foo Bar");

    assert_format_no_errors(bundle.format("placeable-message", None), "Foo Bar");

    assert_format_no_errors(bundle.format("selector-literal", None), "Member 1");

    assert_format_no_errors(bundle.format("bar.attr", None), "Bar Attribute");

    assert_format_no_errors(bundle.format("placeable-attr", None), "Bar Attribute");

    assert_format_no_errors(bundle.format("selector-attr", None), "Member 3");
}

#[test]
fn primitives_complex_string() {
    let res = assert_get_resource_from_str_no_errors(
        r#"
foo               = Foo
bar               = { foo }Bar

placeable-message = { bar }Baz

baz =
    .attr = { bar }BazAttribute

-bazTerm = Value
    .attr = { bar }BazAttribute

placeable-attr = { baz.attr }

selector-attr = { -bazTerm.attr ->
    [FooBarBazAttribute] FooBarBaz
   *[other] Other
}
    "#,
    );
    let bundle = assert_get_bundle_no_errors(res, None);

    assert_format_no_errors(bundle.format("bar", None), "FooBar");

    assert_format_no_errors(bundle.format("placeable-message", None), "FooBarBaz");

    assert_format_no_errors(bundle.format("baz.attr", None), "FooBarBazAttribute");

    assert_format_no_errors(bundle.format("placeable-attr", None), "FooBarBazAttribute");

    assert_format_no_errors(bundle.format("selector-attr", None), "FooBarBaz");
}
