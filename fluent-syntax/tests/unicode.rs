use fluent_syntax::unicode::unescape_unicode;

#[test]
fn unescape_unicode_test() {
    assert_eq!(unescape_unicode("foo"), None);
    assert_eq!(unescape_unicode("foo \\\\"), Some("foo \\".to_owned()));
    assert_eq!(unescape_unicode("foo \\\""), Some("foo \"".to_owned()));
    assert_eq!(
        unescape_unicode("foo \\\\ faa"),
        Some("foo \\ faa".to_owned())
    );
    assert_eq!(
        unescape_unicode("foo \\\\ faa \\\\ fii"),
        Some("foo \\ faa \\ fii".to_owned())
    );
    assert_eq!(
        unescape_unicode("foo \\\\\\\" faa \\\"\\\\ fii"),
        Some("foo \\\" faa \"\\ fii".to_owned())
    );
    assert_eq!(unescape_unicode("\\u0041\\u004F"), Some("AO".to_owned()));
    assert_eq!(unescape_unicode("\\uA"), Some("�".to_owned()));
    assert_eq!(unescape_unicode("\\uA0Pl"), Some("�".to_owned()));
    assert_eq!(unescape_unicode("\\d Foo"), Some("� Foo".to_owned()));
}
