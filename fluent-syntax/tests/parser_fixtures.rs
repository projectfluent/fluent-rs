mod helper;

use glob::glob;
use std::fs;
use std::io;

use fluent_syntax::ast;
use fluent_syntax::parser::Parser;

use helper::{adapt_ast, strip_comments};

fn read_file(path: &str, trim: bool) -> Result<String, io::Error> {
    let s = fs::read_to_string(path)?;
    if trim {
        Ok(s.trim().to_string())
    } else {
        Ok(s)
    }
}

// We temporarily blacklist the CRLF test until we improve PartialEq
// between two `Pattern`s with different `TextElement`s.
const BLACKLIST: &[&str] = &["tests/fixtures/crlf.ftl"];

#[test]
fn parse_fixtures_compare() {
    for entry in glob("./tests/fixtures/*.ftl").expect("Failed to read glob pattern") {
        let p = entry.expect("Error while getting an entry");
        let path = p.to_str().expect("Can't print path");
        if BLACKLIST.contains(&path) {
            continue;
        }

        let reference_path = path.replace(".ftl", ".json");
        let reference_file = read_file(&reference_path, true).unwrap();
        let ftl_file = read_file(&path, false).unwrap();

        println!("Parsing: {:#?}", path);
        let target_ast = match Parser::new(ftl_file).parse() {
            Ok(res) => res,
            Err((res, _errors)) => res,
        };

        let mut ref_ast: ast::Resource<String> =
            serde_json::from_str(reference_file.as_str()).unwrap();
        adapt_ast(&mut ref_ast);

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

        let _ = Parser::new(string.as_str()).parse();
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
        let target_ast = match Parser::new(ftl_file).parse() {
            Ok(res) => res,
            Err((res, _errors)) => res,
        };

        let mut ref_ast: ast::Resource<String> =
            serde_json::from_str(reference_file.as_str()).unwrap();
        adapt_ast(&mut ref_ast);

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
            let ftl_file = read_file(&path, false).unwrap();

            println!("Parsing: {:#?}", path);
            let target_ast = match Parser::new(ftl_file.clone()).parse() {
                Ok(res) => res,
                Err((res, _errors)) => res,
            };

            let mut ref_ast: ast::Resource<String> =
                serde_json::from_str(reference_file.as_str()).unwrap();
            adapt_ast(&mut ref_ast);

            assert_eq!(target_ast.body.len(), ref_ast.body.len());
            for (entry, ref_entry) in target_ast.body.iter().zip(ref_ast.body.iter()) {
                assert_eq!(entry, ref_entry);
            }

            // Skipping comments
            let target_ast = match Parser::new(ftl_file).parse_runtime() {
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
