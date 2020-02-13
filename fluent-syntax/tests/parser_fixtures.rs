mod ast;

use assert_json_diff::assert_json_include;
use glob::glob;
use serde_json::Value;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use fluent_syntax::parser::Parser;

const WHITELIST: &[&str] = &[
    "tests/fixtures/any_char.ftl",
    "tests/fixtures/comments.ftl",
    "tests/fixtures/cr.ftl",
    "tests/fixtures/eof_comment.ftl",
    "tests/fixtures/eof_empty.ftl",
    "tests/fixtures/eof_id_equals.ftl",
    "tests/fixtures/eof_id.ftl",
    "tests/fixtures/eof_junk.ftl",
    "tests/fixtures/eof_value.ftl",
    "tests/fixtures/literal_expressions.ftl",
    "tests/fixtures/messages.ftl",
    "tests/fixtures/mixed_entries.ftl",
    "tests/fixtures/multiline_values.ftl",
    "tests/fixtures/whitespace_in_value.ftl",
];

fn compare_jsons(value: &str, reference: &str) {
    let a: Value = serde_json::from_str(value).unwrap();

    let b: Value = serde_json::from_str(reference).unwrap();

    assert_json_include!(actual: a, expected: b);
}

fn read_file(path: &str, trim: bool) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    if trim {
        Ok(s.trim().to_string())
    } else {
        Ok(s)
    }
}

#[test]
fn parse_fixtures_compare() {
    for entry in glob("./tests/fixtures/*.ftl").expect("Failed to read glob pattern") {
        let p = entry.expect("Error while getting an entry");
        let path = p.to_str().expect("Can't print path");
        if !WHITELIST.contains(&path) {
            continue;
        }

        let reference_path = path.replace(".ftl", ".json");
        let reference_file = read_file(&reference_path, true).unwrap();
        let ftl_file = read_file(&path, false).unwrap();

        println!("Parsing: {:#?}", path);
        let parser = Parser::new(&ftl_file);
        let target_ast = parser.parse();

        let target_json = ast::serialize(&target_ast).unwrap();

        compare_jsons(&target_json, &reference_file);
    }
}

#[test]
fn parse_fixtures() {
    for entry in glob("./tests/fixtures/any_char.ftl").expect("Failed to read glob pattern") {
        let p = entry.expect("Error while getting an entry");
        let path = p.to_str().expect("Can't print path");

        println!("Attempting to parse file: {}", path);

        let string = read_file(path, false).expect("Failed to read");

        let parser = Parser::new(&string);
        let _ = parser.parse();
    }
}
