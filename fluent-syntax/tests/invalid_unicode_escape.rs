use fluent_syntax::parser::{parse, parse_runtime, ErrorKind};

/// An invalid `\u`/`\U` escape followed by a multi-byte character must be a
/// parse error, not a panic from slicing inside that character (#389).
#[test]
fn invalid_unicode_escape_before_multibyte_char() {
    let cases = [
        ("a = {\"\\u\u{30c}\"}", "\u{30c}"),
        ("a = {\"\\uǔ\"}", "ǔ"),
        ("a = {\"\\u12ǔ\"}", "12ǔ"),
        ("a = {\"\\U1234😀\"}", "1234😀"),
        ("a={\"\\u\u{30c}", "\u{30c}"),
    ];
    for (source, seq) in cases {
        let expected = ErrorKind::InvalidUnicodeEscapeSequence(seq.to_string());
        for result in [parse(source), parse_runtime(source)] {
            let (_, errors) = result.expect_err(source);
            assert_eq!(errors[0].kind, expected, "{source}");
        }
        let (_, errors) = parse(source.to_string()).expect_err(source);
        assert_eq!(errors[0].kind, expected, "{source}");
    }
}
