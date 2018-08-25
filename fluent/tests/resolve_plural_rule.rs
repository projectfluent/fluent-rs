extern crate fluent;

use std::collections::HashMap;

use self::fluent::context::FluentBundle;
use self::fluent::types::FluentValue;

#[test]
fn external_argument_number() {
    let mut bundle = FluentBundle::new(&["en"]);
    bundle.add_messages(
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

    let value = bundle.format("unread-emails", Some(&args));
    assert_eq!(value, Some("You have 1 unread email.".to_string()));

    let value = bundle.format("unread-emails-dec", Some(&args));
    assert_eq!(value, Some("You have 1.0 unread emails.".to_string()));
}

#[test]
fn exact_match() {
    let mut bundle = FluentBundle::new(&["en"]);
    bundle.add_messages(
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

    let value = bundle.format("unread-emails", Some(&args));
    assert_eq!(value, Some("You have one unread email.".to_string()));

    let value = bundle.format("unread-emails-dec", Some(&args));
    assert_eq!(value, Some("You have one unread email.".to_string()));
}
