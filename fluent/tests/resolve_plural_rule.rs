extern crate fluent;

use std::collections::HashMap;

use self::fluent::context::MessageContext;
use self::fluent::types::FluentValue;

#[test]
fn external_argument_number() {
    let mut ctx = MessageContext::new(&["en"]);

    ctx.add_messages(
        "
unread-emails =
    { $emailsCount ->
        [one] You have { $emailsCount } unread email.
       *[other] You have { $emailsCount } unread emails.
    }

unread-emails-dec =
    { $emailsCountDec ->
        [one] You have { $emailsCountDec } unread email.
       *[other] You have { $emailsCountDec } unread emails.
    }

",
    );

    let mut args = HashMap::new();
    args.insert("emailsCount", FluentValue::from(1));
    args.insert("emailsCountDec", FluentValue::as_number("1.0").unwrap());

    let value = ctx
        .get_message("unread-emails")
        .and_then(|msg| ctx.format(msg, Some(&args)));

    assert_eq!(value, Some("You have 1 unread email.".to_string()));

    let value = ctx
        .get_message("unread-emails-dec")
        .and_then(|msg| ctx.format(msg, Some(&args)));

    assert_eq!(value, Some("You have 1.0 unread emails.".to_string()));
}

#[test]
fn exact_match() {
    let mut ctx = MessageContext::new(&["en"]);

    ctx.add_messages(
        "
unread-emails =
    { $emailsCount ->
        [1] You have one unread email.
        [one] You have { $emailsCount } unread email.
       *[other] You have { $emailsCount } unread emails.
    }

unread-emails-dec =
    { $emailsCountDec ->
        [1.0] You have one unread email.
        [one] You have { $emailsCountDec } unread email.
       *[other] You have { $emailsCountDec } unread emails.
    }

",
    );

    let mut args = HashMap::new();
    args.insert("emailsCount", FluentValue::from(1));
    args.insert("emailsCountDec", FluentValue::as_number("1.0").unwrap());

    let value = ctx
        .get_message("unread-emails")
        .and_then(|msg| ctx.format(msg, Some(&args)));

    assert_eq!(value, Some("You have one unread email.".to_string()));

    let value = ctx
        .get_message("unread-emails-dec")
        .and_then(|msg| ctx.format(msg, Some(&args)));

    assert_eq!(value, Some("You have one unread email.".to_string()));
}
