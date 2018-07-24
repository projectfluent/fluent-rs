extern crate fluent;

use self::fluent::context::MessageContext;

#[test]
fn message_reference() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages(
        "
foo = Foo
bar = { foo } Bar
",
    );

    let value = ctx.get_message("bar").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Foo Bar".to_string()));
}

#[test]
fn term_reference() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages(
        "
-foo = Foo
bar = { -foo } Bar
",
    );

    let value = ctx.get_message("bar").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Foo Bar".to_string()));
}

#[test]
fn message_reference_nested() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages(
        "
foo = Foo
bar = { foo } Bar
baz = { bar } Baz
",
    );

    let value = ctx.get_message("baz").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Foo Bar Baz".to_string()));
}

#[test]
fn message_reference_missing() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages("bar = { foo } Bar");

    let value = ctx.get_message("bar").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("___ Bar".to_string()));
}

#[test]
fn message_reference_cyclic() {
    {
        let mut ctx = MessageContext::new(&["x-testing"]);

        ctx.add_messages(
            "
foo = Foo { bar }
bar = { foo } Bar
",
        );

        let value = ctx.get_message("foo").and_then(|msg| ctx.format(msg, None));
        assert_eq!(value, Some("Foo ___".to_string()));
        let value = ctx.get_message("bar").and_then(|msg| ctx.format(msg, None));
        assert_eq!(value, Some("___ Bar".to_string()));
    }

    {
        let mut ctx = MessageContext::new(&["x-testing"]);

        ctx.add_messages(
            "
foo = { bar }
bar = { foo }
",
        );

        let value = ctx.get_message("foo").and_then(|msg| ctx.format(msg, None));
        assert_eq!(value, Some("___".to_string()));
        let value = ctx.get_message("bar").and_then(|msg| ctx.format(msg, None));
        assert_eq!(value, Some("___".to_string()));
    }
}

#[test]
fn message_reference_multiple() {
    let mut ctx = MessageContext::new(&["x-testing"]);

    ctx.add_messages(
        "
foo = Foo
bar = { foo } Bar { foo }
",
    );

    let value = ctx.get_message("bar").and_then(|msg| ctx.format(msg, None));
    assert_eq!(value, Some("Foo Bar Foo".to_string()));
}
