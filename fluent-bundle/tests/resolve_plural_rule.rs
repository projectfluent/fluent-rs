mod helpers;

use std::collections::HashMap;

use self::helpers::{assert_add_messages_no_errors, assert_format_no_errors};
use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::types::FluentValue;

#[test]
fn external_argument_number() {
    let mut bundle = FluentBundle::new(&["en"]);
    assert_add_messages_no_errors(bundle.add_messages(
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
    ));

    let mut args = HashMap::new();
    args.insert("emailsCount", FluentValue::from(1));
    args.insert("emailsCountDec", FluentValue::as_number("1.0").unwrap());

    assert_format_no_errors(
        bundle.format("unread-emails", Some(&args)),
        "You have 1 unread email.",
    );

    assert_format_no_errors(
        bundle.format("unread-emails-dec", Some(&args)),
        "You have 1.0 unread emails.",
    );
}

#[test]
fn exact_match() {
    let mut bundle = FluentBundle::new(&["en"]);
    assert_add_messages_no_errors(bundle.add_messages(
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
    ));

    let mut args = HashMap::new();
    args.insert("emailsCount", FluentValue::from(1));
    args.insert("emailsCountDec", FluentValue::as_number("1.0").unwrap());

    assert_format_no_errors(
        bundle.format("unread-emails", Some(&args)),
        "You have one unread email.",
    );

    assert_format_no_errors(
        bundle.format("unread-emails-dec", Some(&args)),
        "You have one unread email.",
    );
}
