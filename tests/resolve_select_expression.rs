extern crate fluent;

use std::collections::HashMap;

use self::fluent::context::MessageContext;
use self::fluent::types::FluentValue;

#[test]
fn select_expression_without_selector() {
    let mut ctx = MessageContext::new("x-testing");

    ctx.add_messages(
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
    );

    let value = ctx.get_message("foo").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Foo".to_string()));

    let value = ctx.get_message("bar").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Bar".to_string()));
}

#[test]
fn select_expression_string_selector() {
    let mut ctx = MessageContext::new("x-testing");

    ctx.add_messages(
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

    let value = ctx.get_message("foo").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Foo's".to_string()));

    let value = ctx.get_message("bar").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Bar".to_string()));
}

#[test]
fn select_expression_number_selector() {
    let mut ctx = MessageContext::new("x-testing");

    ctx.add_messages(
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
       *[1] Bar 1
        [3] Bar 3
        [3.14] Bar Pi
    }
",
    );

    let value = ctx.get_message("foo").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Foo 3".to_string()));

    let value = ctx.get_message("bar").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Bar 1".to_string()));

    let value = ctx.get_message("baz").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Bar Pi".to_string()));
}

#[test]
fn select_expression_external_argument_selector() {
    let mut ctx = MessageContext::new("x-testing");

    ctx.add_messages(
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

    let mut args = HashMap::new();
    args.insert("str", FluentValue::from("qux"));
    args.insert("int", FluentValue::from(3));
    args.insert("float", FluentValue::from(2.72));

    let value = ctx.get_message("foo-hit").and_then(
        |msg| ctx.format(msg, Some(&args)),
    );
    assert_eq!(value, Some("Qux".to_string()));

    let value = ctx.get_message("foo-miss").and_then(|msg| {
        ctx.format(msg, Some(&args))
    });
    assert_eq!(value, Some("Foo".to_string()));

    let value = ctx.get_message("foo-unknown").and_then(|msg| {
        ctx.format(msg, Some(&args))
    });
    assert_eq!(value, Some("Foo".to_string()));

    let value = ctx.get_message("bar-hit").and_then(
        |msg| ctx.format(msg, Some(&args)),
    );
    assert_eq!(value, Some("Bar 3".to_string()));

    let value = ctx.get_message("bar-miss").and_then(|msg| {
        ctx.format(msg, Some(&args))
    });
    assert_eq!(value, Some("Bar 1".to_string()));

    let value = ctx.get_message("bar-unknown").and_then(|msg| {
        ctx.format(msg, Some(&args))
    });
    assert_eq!(value, Some("Bar 1".to_string()));

    let value = ctx.get_message("baz-hit").and_then(
        |msg| ctx.format(msg, Some(&args)),
    );
    assert_eq!(value, Some("Baz E".to_string()));

    let value = ctx.get_message("baz-miss").and_then(|msg| {
        ctx.format(msg, Some(&args))
    });
    assert_eq!(value, Some("Baz 1".to_string()));

    let value = ctx.get_message("baz-unknown").and_then(|msg| {
        ctx.format(msg, Some(&args))
    });
    assert_eq!(value, Some("Baz 1".to_string()));
}

#[test]
fn select_expression_message_selector() {
    let mut ctx = MessageContext::new("x-testing");

    ctx.add_messages(
        "
foo = Foo
bar = Bar
    #tag
baz = Baz
    #tag1
    #tag2

use-foo =
    { foo ->
        [Foo] Foo
       *[other] Other
    }

use-bar =
    { bar ->
        [tag] Bar
       *[other] Other
    }

use-baz =
    { baz ->
        [tag2] Baz 2
        [tag1] Baz 1
       *[other] Other
    }

",
    );

    let value = ctx.get_message("use-foo").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Other".to_string()));

    let value = ctx.get_message("use-bar").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Bar".to_string()));

    let value = ctx.get_message("use-baz").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Baz 2".to_string()));
}
