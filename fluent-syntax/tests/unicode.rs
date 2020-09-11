use fluent_syntax::unicode::unescape_unicode;

fn test_unescape_unicode(input: &str, output: &str) {
    let mut s = String::new();
    unescape_unicode(&mut s, input).expect("Failed to write.");
    assert_eq!(&s, output);
}

#[test]
fn unescape_unicode_test() {
    test_unescape_unicode("foo", "foo");
    test_unescape_unicode("foo \\\\", "foo \\");
    // test_unescape_unicode("foo \\\"", "foo \"");
    // test_unescape_unicode("foo \\\\ faa", "foo \\ faa");
    // test_unescape_unicode("foo \\\\ faa \\\\ fii",
    //     "foo \\ faa \\ fii"
    // );
    // test_unescape_unicode("foo \\\\\\\" faa \\\"\\\\ fii",
    //     "foo \\\" faa \"\\ fii"
    // );
    // test_unescape_unicode("\\u0041\\u004F", "AO");
    // test_unescape_unicode("\\uA", "�");
    // test_unescape_unicode("\\uA0Pl", "�");
    // test_unescape_unicode("\\d Foo", "� Foo");
}
