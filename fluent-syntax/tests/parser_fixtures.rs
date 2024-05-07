mod helper;

use glob::glob;
use std::fs;
use std::io;

use fluent_syntax::ast;
use fluent_syntax::parser::{parse, parse_runtime};

use helper::{adapt_ast, strip_comments};

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
        let is_crlf = path.contains("crlf");

        let reference_path = path.replace(".ftl", ".json");
        let reference_file = read_file(&reference_path, true).unwrap();
        let ftl_file = read_file(path, false).unwrap();

        println!("Parsing: {:#?}", path);
        let target_ast = match parse(ftl_file) {
            Ok(res) => res,
            Err((res, _errors)) => res,
        };

        let mut ref_ast: ast::Resource<String> =
            serde_json::from_str(reference_file.as_str()).unwrap();
        adapt_ast(&mut ref_ast, is_crlf);

        assert_eq!(target_ast.body.len(), ref_ast.body.len());
        for (entry, ref_entry) in target_ast.body.iter().zip(ref_ast.body.iter()) {
            assert_eq!(entry, ref_entry);
        }
    }
}

#[test]
fn parse_fixtures() {
    for entry in glob("./tests/fixtures/*.ftl").expect("Failed to read glob pattern") {
        let p = entry.expect("Error while getting an entry");
        let path = p.to_str().expect("Can't print path");

        println!("Attempting to parse file: {}", path);

        let string = read_file(path, false).expect("Failed to read");

        let _ = parse(string.as_str());
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
        let ftl_file = read_file(path, false).unwrap();

        println!("Parsing: {:#?}", path);
        let target_ast = match parse(ftl_file) {
            Ok(res) => res,
            Err((res, _errors)) => res,
        };

        let mut ref_ast: ast::Resource<String> =
            serde_json::from_str(reference_file.as_str()).unwrap();
        adapt_ast(&mut ref_ast, false);

        assert_eq!(target_ast.body.len(), ref_ast.body.len());
        for (entry, ref_entry) in target_ast.body.iter().zip(ref_ast.body.iter()) {
            assert_eq!(entry, ref_entry);
        }
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
            let ftl_file = read_file(path, false).unwrap();

            println!("Parsing: {:#?}", path);
            let target_ast = match parse(ftl_file.clone()) {
                Ok(res) => res,
                Err((res, _errors)) => res,
            };

            let mut ref_ast: ast::Resource<String> =
                serde_json::from_str(reference_file.as_str()).unwrap();
            adapt_ast(&mut ref_ast, false);

            assert_eq!(target_ast.body.len(), ref_ast.body.len());
            for (entry, ref_entry) in target_ast.body.iter().zip(ref_ast.body.iter()) {
                assert_eq!(entry, ref_entry);
            }

            // Skipping comments
            let target_ast = match parse_runtime(ftl_file) {
                Ok(res) => res,
                Err((res, _errors)) => res,
            };

            strip_comments(&mut ref_ast);

            assert_eq!(target_ast.body.len(), ref_ast.body.len());
            for (entry, ref_entry) in target_ast.body.iter().zip(ref_ast.body.iter()) {
                assert_eq!(entry, ref_entry);
            }
        }
    }
}
