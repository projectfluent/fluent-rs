use std::borrow::Cow;

use fluent_syntax::unicode::{unescape_unicode, unescape_unicode_to_string};

/// Asserts that decoding unicode escape sequences in `input` matches `output`.
/// When `borrowed` = true, asserts that the escaped value is passed back by reference.
fn test_unescape_unicode(input: &str, output: &str, borrowed: bool) {
    let mut s = String::new();
    unescape_unicode(&mut s, input).expect("Failed to write.");
    assert_eq!(s, output);
    let result = unescape_unicode_to_string(input);
    assert_eq!(result, output);

    assert_eq!(matches!(result, Cow::Borrowed(_)), borrowed);
}

#[test]
fn unescape_unicode_test() {
    test_unescape_unicode("foo", "foo", true);
    test_unescape_unicode("foo \\\\", "foo \\", false);
    test_unescape_unicode("foo \\\"", "foo \"", false);
    test_unescape_unicode("foo \\\\ faa", "foo \\ faa", false);
    test_unescape_unicode("foo \\\\ faa \\\\ fii", "foo \\ faa \\ fii", false);
    test_unescape_unicode(
        "foo \\\\\\\" faa \\\"\\\\ fii",
        "foo \\\" faa \"\\ fii",
        false,
    );
    test_unescape_unicode("\\u0041\\u004F", "AO", false);
    test_unescape_unicode("\\uA", "�", false);
    test_unescape_unicode("\\uA0Pl", "�", false);
    test_unescape_unicode("\\d Foo", "� Foo", false);
}
