mod helpers;
use fluent_bundle::errors::FluentError;
use fluent_bundle::resolve::ResolverError;

use std::collections::HashMap;

use self::helpers::{
    assert_format, assert_format_no_errors, assert_get_bundle_no_errors,
    assert_get_resource_from_str_no_errors,
};
use fluent_bundle::types::FluentValue;

#[test]
fn select_expression_string_selector() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo =
    { \"genitive\" ->
       *[nominative] Foo
        [genitive] Foo's
    }

bar =
    { \"missing\" ->
       *[nominative] Bar
        [genitive] Bar's
    }
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_format_no_errors(bundle.format("foo", None), "Foo's");

    assert_format_no_errors(bundle.format("bar", None), "Bar");
}

#[test]
fn select_expression_number_selector() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo =
    { 3 ->
       *[1] Foo 1
        [3] Foo 3
    }

bar =
    { 3 ->
       *[1] Bar 1
        [2] Bar 2
    }

baz =
    { 3.14 ->
       *[1] Baz 1
        [3] Baz 3
        [3.14] Baz Pi
    }
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_format_no_errors(bundle.format("foo", None), "Foo 3");

    assert_format_no_errors(bundle.format("bar", None), "Bar 1");

    assert_format_no_errors(bundle.format("baz", None), "Baz Pi");
}

#[test]
fn select_expression_plurals() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo =
    { 3 ->
        [one] Foo One
        [3] Foo 3
       *[other] Foo Other
    }

bar =
    { 1 ->
        [one] Bar One
        [2] Bar 2
       *[other] Bar Other
    }

baz =
    { \"one\" ->
        [1] Bar One
        [3] Bar 3
       *[other] Bar Other
    }
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, Some("en"));

    assert_format_no_errors(bundle.format("foo", None), "Foo 3");

    assert_format_no_errors(bundle.format("bar", None), "Bar One");

    assert_format_no_errors(bundle.format("baz", None), "Bar Other");
}

#[test]
fn select_expression_external_argument_selector() {
    let res = assert_get_resource_from_str_no_errors(
        "
foo-hit =
    { $str ->
       *[foo] Foo
        [qux] Qux
    }

foo-miss =
    { $str ->
       *[foo] Foo
        [bar] Bar
    }

foo-unknown =
    { $unknown ->
       *[foo] Foo
        [bar] Bar
    }

bar-hit =
    { $int ->
       *[1] Bar 1
        [3] Bar 3
    }

bar-miss =
    { $int ->
       *[1] Bar 1
        [2] Bar 2
    }

bar-unknown =
    { $unknown ->
       *[1] Bar 1
        [2] Bar 2
    }

baz-hit =
    { $float ->
       *[1] Baz 1
        [2.72] Baz E
    }

baz-miss =
    { $float ->
       *[1] Baz 1
        [2] Baz 2
    }

baz-unknown =
    { $unknown ->
       *[1] Baz 1
        [2] Baz 2
    }
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    let mut args = HashMap::new();
    args.insert("str", FluentValue::from("qux"));
    args.insert("int", FluentValue::from(3));
    args.insert("float", FluentValue::from(2.72));

    assert_format_no_errors(bundle.format("foo-hit", Some(&args)), "Qux");

    assert_format_no_errors(bundle.format("foo-miss", Some(&args)), "Foo");

    assert_format(
        bundle.format("foo-unknown", Some(&args)),
        "Foo",
        vec![FluentError::ResolverError(ResolverError::Reference(
            "Unknown variable: $unknown".into(),
        ))],
    );

    assert_format_no_errors(bundle.format("bar-hit", Some(&args)), "Bar 3");

    assert_format_no_errors(bundle.format("bar-miss", Some(&args)), "Bar 1");

    assert_format(
        bundle.format("bar-unknown", Some(&args)),
        "Bar 1",
        vec![FluentError::ResolverError(ResolverError::Reference(
            "Unknown variable: $unknown".into(),
        ))],
    );

    assert_format_no_errors(bundle.format("baz-hit", Some(&args)), "Baz E");

    assert_format_no_errors(bundle.format("baz-miss", Some(&args)), "Baz 1");

    assert_format(
        bundle.format("baz-unknown", Some(&args)),
        "Baz 1",
        vec![FluentError::ResolverError(ResolverError::Reference(
            "Unknown variable: $unknown".into(),
        ))],
    );
}

#[test]
fn select_expression_message_selector() {
    let res = assert_get_resource_from_str_no_errors(
        "
-bar = Bar
    .attr = attr_val

use-bar =
    { -bar.attr ->
        [attr_val] Bar
       *[other] Other
    }
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_format_no_errors(bundle.format("use-bar", None), "Bar");
}

#[test]
fn select_expression_attribute_selector() {
    let res = assert_get_resource_from_str_no_errors(
        "
-foo = Foo
    .attr = FooAttr

use-foo =
    { -foo.attr ->
        [FooAttr] Foo
       *[other] Other
    }
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_format_no_errors(bundle.format("use-foo", None), "Foo");
}
