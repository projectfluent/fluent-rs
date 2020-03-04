use assert_json_diff::assert_json_include;
use glob::glob;
use serde_json::Value;
use std::fs;
use std::io;

use fluent_syntax::json;
use fluent_syntax::parser::parse;

fn compare_jsons(value: &str, reference: &str) {
    let a: Value = serde_json::from_str(value).unwrap();

    let b: Value = serde_json::from_str(reference).unwrap();

    assert_json_include!(actual: a, expected: b);
}

fn read_file(path: &str, trim: bool) -> Result<String, io::Error> {
    let s = fs::read_to_string(path)?;
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

        let reference_path = path.replace(".ftl", ".json");
        let reference_file = read_file(&reference_path, true).unwrap();
        let ftl_file = read_file(&path, false).unwrap();

        println!("Parsing: {:#?}", path);
        let target_ast = match parse(&ftl_file) {
            Ok(res) => res,
            Err((res, _errors)) => res,
        };

        let target_json = json::serialize(&target_ast).unwrap();

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

#[test]
fn parse_bench_fixtures() {
    for entry in glob("./benches/*.ftl").expect("Failed to read glob pattern") {
        let p = entry.expect("Error while getting an entry");
        let path = p.to_str().expect("Can't print path");
        let file_name = p.file_name().unwrap().to_str().unwrap();

        let reference_path = format!(
            "./tests/fixtures/benches/{}",
            file_name.replace(".ftl", ".json")
        );
        let reference_file = read_file(&reference_path, true).unwrap();
        let ftl_file = read_file(&path, false).unwrap();

        println!("Parsing: {:#?}", path);
        let target_ast = match parse(&ftl_file) {
            Ok(res) => res,
            Err((res, _errors)) => res,
        };

        let target_json = json::serialize(&target_ast).unwrap();

        compare_jsons(&target_json, &reference_file);
    }

    let contexts = &["browser", "preferences"];

    for context in contexts {
        for entry in glob(&format!("./benches/contexts/{}/*.ftl", context))
            .expect("Failed to read glob pattern")
        {
            let p = entry.expect("Error while getting an entry");
            let path = p.to_str().expect("Can't print path");
            let file_name = p.file_name().unwrap().to_str().unwrap();

            let reference_path = format!(
                "./tests/fixtures/benches/contexts/{}/{}",
                context,
                file_name.replace(".ftl", ".json")
            );
            let reference_file = read_file(&reference_path, true).unwrap();
            let ftl_file = read_file(&path, false).unwrap();

            println!("Parsing: {:#?}", path);
            let target_ast = match parse(&ftl_file) {
                Ok(res) => res,
                Err((res, _errors)) => res,
            };

            let target_json = json::serialize(&target_ast).unwrap();

            compare_jsons(&target_json, &reference_file);
        }
    }
}
