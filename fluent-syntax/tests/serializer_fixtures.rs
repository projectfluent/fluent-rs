use fluent_syntax::ast::{Entry, Resource};
use glob::glob;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use fluent_syntax::parser::parse;
use fluent_syntax::serializer::{serialize, serialize_with_options, Options};

/// List of files that currently do not roundtrip correctly.
///
/// - `multiline_values.ftl`: `key12` is parsed differently if indented.
const BLACKLIST: [&str; 1] = ["multiline_values.ftl"];

fn is_blacklisted(path: &Path) -> bool {
    path.file_name()
        .and_then(OsStr::to_str)
        .map(|s| BLACKLIST.contains(&s))
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
        if is_blacklisted(&path) {
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
