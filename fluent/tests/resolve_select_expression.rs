extern crate fluent;

mod helpers;

use std::collections::HashMap;

use self::fluent::context::FluentBundle;
use self::fluent::types::FluentValue;
use helpers::{assert_add_messages_no_errors, assert_format_no_errors};

#[test]
fn select_expression_without_selector() {
    let mut bundle = FluentBundle::new(&["x-testing"]);

    assert_add_messages_no_errors(bundle.add_messages(
        "
foo =
    {
       *[nominative] Foo
        [genitive] Foo's
    }

bar =
    {
        [genitive] Bar's
       *[nominative] Bar
    }
",
    ));

    assert_format_no_errors(bundle.format("foo", None), "Foo");

    assert_format_no_errors(bundle.format("bar", None), "Bar");
}

#[test]
fn select_expression_string_selector() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages(
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
    ));

    assert_format_no_errors(bundle.format("foo", None), "Foo's");

    assert_format_no_errors(bundle.format("bar", None), "Bar");
}

#[test]
fn select_expression_number_selector() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages(
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
    ));

    assert_format_no_errors(bundle.format("foo", None), "Foo 3");

    assert_format_no_errors(bundle.format("bar", None), "Bar 1");

    assert_format_no_errors(bundle.format("baz", None), "Baz Pi");
}

#[test]
fn select_expression_plurals() {
    let mut bundle = FluentBundle::new(&["en"]);
    assert_add_messages_no_errors(bundle.add_messages(
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
    ));

    assert_format_no_errors(bundle.format("foo", None), "Foo 3");

    assert_format_no_errors(bundle.format("bar", None), "Bar One");

    assert_format_no_errors(bundle.format("baz", None), "Bar Other");
}

#[test]
fn select_expression_external_argument_selector() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages(
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
    ));

    let mut args = HashMap::new();
    args.insert("str", FluentValue::from("qux"));
    args.insert("int", FluentValue::from(3));
    args.insert("float", FluentValue::from(2.72));

    assert_format_no_errors(bundle.format("foo-hit", Some(&args)), "Qux");

    assert_format_no_errors(bundle.format("foo-miss", Some(&args)), "Foo");

    assert_format_no_errors(bundle.format("foo-unknown", Some(&args)), "Foo");

    assert_format_no_errors(bundle.format("bar-hit", Some(&args)), "Bar 3");

    assert_format_no_errors(bundle.format("bar-miss", Some(&args)), "Bar 1");

    assert_format_no_errors(bundle.format("bar-unknown", Some(&args)), "Bar 1");

    assert_format_no_errors(bundle.format("baz-hit", Some(&args)), "Baz E");

    assert_format_no_errors(bundle.format("baz-miss", Some(&args)), "Baz 1");

    assert_format_no_errors(bundle.format("baz-unknown", Some(&args)), "Baz 1");
}

#[test]
fn select_expression_message_selector() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages(
        "
-bar = Bar
    .attr = attr val

use-bar =
    { -bar.attr ->
        [attr val] Bar
       *[other] Other
    }
",
    ));

    assert_format_no_errors(bundle.format("use-bar", None), "Bar");
}

#[test]
fn select_expression_attribute_selector() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    assert_add_messages_no_errors(bundle.add_messages(
        "
-foo = Foo
    .attr = Foo Attr

use-foo =
    { -foo.attr ->
        [Foo Attr] Foo
       *[other] Other
    }
",
    ));

    assert_format_no_errors(bundle.format("use-foo", None), "Foo");
}
