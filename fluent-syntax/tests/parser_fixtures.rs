mod ast;

use glob::glob;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::Command;

use fluent_syntax::parser::parse;

fn compare_jsons(value: &str, reference: &str) -> String {
    let temp_path = reference.replace(".json", ".candidate.json");
    write_file(&temp_path, value).unwrap();

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("json-diff {} {}", reference, &temp_path))
        .output()
        .expect("failed to execute process");
    let s = String::from_utf8_lossy(&output.stdout).to_string();
    fs::remove_file(&temp_path).unwrap();
    s
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

fn write_file(path: &str, value: &str) -> std::io::Result<()> {
    let mut file = File::create(&path)?;
    file.write_all(value.as_bytes())?;
    Ok(())
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

        let target_json = ast::serialize(&target_ast).unwrap();

        let diff = compare_jsons(&target_json, &reference_path);
        assert_eq!(
            reference_file, target_json,
            "\n=====\nThe diff {} :\n-------\n{}\n-----\n",
            path, diff
        );
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
