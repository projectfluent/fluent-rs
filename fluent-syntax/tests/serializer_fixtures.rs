use fluent_syntax::ast::{Entry, Resource};
use glob::glob;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use fluent_syntax::parser::parse;
use fluent_syntax::serializer::{serialize, serialize_with_options, Options};

/// List of files that currently do not roundtrip correctly.
///
/// - `multiline_values.ftl`: <https://github.com/projectfluent/fluent-rs/issues/286>
/// - `crlf.ftl`: Parsing `foo =\r\n    bar\r\n    baz\r\n` results in a `TextElement` "bar" and a `TextElement` "\n",
///   whereas parsing `foo =\n    bar\n    baz\n` results in a single `TextElement` "bar\n". That means resources with
///   text separated by CRLF do not roundtrip correctly.
const IGNORE_LIST: [&str; 2] = ["crlf.ftl", "multiline_values.ftl"];

fn is_ignored(path: &Path) -> bool {
    path.file_name()
        .and_then(OsStr::to_str)
        .map(|s| IGNORE_LIST.contains(&s))
        .unwrap_or_default()
}

fn clone_without_junk<'a>(original: &Resource<&'a str>) -> Resource<&'a str> {
    Resource {
        body: original
            .body
            .iter()
            .filter(|entry| !matches!(entry, Entry::Junk { .. }))
            .cloned()
            .collect(),
        #[cfg(feature = "spans")]
        span: original.span,
    }
}

#[test]
fn roundtrip_normalized_fixtures() {
    for entry in glob("./tests/fixtures/normalized/*.ftl").expect("Failed to read glob pattern") {
        let path = entry.expect("Error while getting an entry");
        let content = fs::read_to_string(&path).expect("Failed to read file");
        let parsed = parse(content.as_str()).unwrap_or_else(|(res, _)| res);
        let reserialized = serialize(&parsed);
        assert_eq!(content, reserialized);
    }
}

/// Compares a parsed AST with a parsed, serialized and reparsed AST, as these fixtures
/// contain unnormalized syntax that is not supposed to be preserved on a roundtrip.
/// Tests both parsing with and without junk.
#[test]
fn roundtrip_unnormalized_fixtures() {
    for entry in glob("./tests/fixtures/*.ftl").expect("Failed to read glob pattern") {
        let path = entry.expect("Error while getting an entry");
        if is_ignored(&path) {
            continue;
        }

        let content = fs::read_to_string(&path).expect("Failed to read file");
        let parsed = parse(content.as_str()).unwrap_or_else(|(res, _)| res);
        let parsed_without_junk = clone_without_junk(&parsed);
        let reserialized = serialize_with_options(&parsed, Options { with_junk: true });
        let reserialized_without_junk =
            serialize_with_options(&parsed, Options { with_junk: false });
        let reparsed = parse(reserialized.as_str()).unwrap_or_else(|(res, _)| res);
        let reparsed_without_junk =
            parse(reserialized_without_junk.as_str()).unwrap_or_else(|(res, _)| res);

        assert_eq!(reparsed_without_junk, parsed_without_junk);
        assert_eq!(reparsed, parsed);
    }
}
