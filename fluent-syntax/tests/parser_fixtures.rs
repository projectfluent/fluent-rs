mod ast;

use assert_json_diff::assert_json_include;
use glob::glob;
use serde_json::Value;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use fluent_syntax::parser::parse;
use fluent_syntax::parser2::Parser;

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
    for entry in glob("./tests/fixtures/any_char.ftl").expect("Failed to read glob pattern") {
        let p = entry.expect("Error while getting an entry");
        let path = p.to_str().expect("Can't print path");

        let reference_path = path.replace(".ftl", ".json");
        let reference_file = read_file(&reference_path, true).unwrap();
        let ftl_file = read_file(&path, false).unwrap();

        println!("Parsing: {:#?}", path);
        ast::set_source(ftl_file.clone());
        let parser = Parser::new(ftl_file.as_bytes());
        let target_ast = parser.parse();

        let target_json = ast::serialize(&target_ast).unwrap();
        println!("{:#?}", target_json);

        compare_jsons(&target_json, &reference_file);
    }
}

#[test]
fn parse_fixtures() {
    for entry in glob("./tests/fixtures/*.ftl").expect("Failed to read glob pattern") {
        let p = entry.expect("Error while getting an entry");
        let path = p.to_str().expect("Can't print path");

        println!("Attempting to parse file: {}", path);

        let string = read_file(path, false).expect("Failed to read");

        let _ = parse(&string);
    }
}
