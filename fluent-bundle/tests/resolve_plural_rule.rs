mod helpers;

use std::collections::HashMap;

use self::helpers::{
    assert_format_no_errors, assert_get_bundle_no_errors, assert_get_resource_from_str_no_errors,
};
use fluent_bundle::types::FluentValue;

#[test]
fn external_argument_number() {
    let res = assert_get_resource_from_str_no_errors(
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
    let bundle = assert_get_bundle_no_errors(res, Some("en"));

    let mut args = HashMap::new();
    args.insert("emailsCount", FluentValue::from(1));
    args.insert("emailsCountDec", FluentValue::into_number("1.0"));

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
    let res = assert_get_resource_from_str_no_errors(
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
    let bundle = assert_get_bundle_no_errors(res, Some("en"));

    let mut args = HashMap::new();
    args.insert("emailsCount", FluentValue::from(1));
    args.insert("emailsCountDec", FluentValue::into_number("1.0"));

    assert_format_no_errors(
        bundle.format("unread-emails", Some(&args)),
        "You have one unread email.",
    );

    assert_format_no_errors(
        bundle.format("unread-emails-dec", Some(&args)),
        "You have one unread email.",
    );
}
