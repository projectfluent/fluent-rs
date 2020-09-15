use assert_json_diff::assert_json_include;
use glob::glob;
use serde_json::Value;
use std::fs;
use std::io;

use fluent_syntax::ast;
use fluent_syntax::parser::Parser;

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

const BLACKLIST: &[&str] = &["tests/fixtures/crlf.ftl"];

const WHITELIST: &[&str] = &[
    // "tests/fixtures/any_char.ftl",
    // "tests/fixtures/astral.ftl",
    // "tests/fixtures/call_expressions.ftl",
    // "tests/fixtures/callee_expressions.ftl",
    // "tests/fixtures/comments.ftl",
    // "tests/fixtures/cr.ftl",
    "tests/fixtures/crlf.ftl",
    // "tests/fixtures/eof_comment.ftl",
    // "tests/fixtures/eof_empty.ftl",
    // "tests/fixtures/eof_id_equals.ftl",
    // "tests/fixtures/eof_id.ftl",
    // "tests/fixtures/eof_junk.ftl",
    // "tests/fixtures/eof_value.ftl",
    // "tests/fixtures/escaped_characters.ftl",
    // "tests/fixtures/junk.ftl",
    // "tests/fixtures/leading_dots.ftl",
    // "tests/fixtures/literal_expression.ftl",
    // "tests/fixtures/messages.ftl",
    // "tests/fixtures/select_indent.ftl",
];

fn adapt_comment(comment: &mut ast::Comment<String>) {
    let mut content = vec![];
    for line in &comment.content {
        content.extend(line.split('\n').map(|s| s.to_string()));
    }
    comment.content = content;
}

fn adapt_pattern(pattern: &mut ast::Pattern<String>) {
    let mut elements = vec![];
    for element in &pattern.elements {
        match element {
            ast::PatternElement::TextElement { value } => {
                let mut start = 0;
                let len = value.as_bytes().len();
                for (i, b) in value.as_bytes().iter().enumerate() {
                    if b == &b'\n' {
                        let chunk = &value.as_bytes()[start..=i];
                        let value = String::from_utf8_lossy(chunk).to_string();
                        elements.push(ast::PatternElement::TextElement { value });
                        start = i + 1;
                    }
                }
                if start < len {
                    let chunk = &value.as_bytes()[start..len];
                    let value = String::from_utf8_lossy(chunk).to_string();
                    elements.push(ast::PatternElement::TextElement { value });
                }
            }
            ast::PatternElement::Placeable { expression } => {
                let mut expression = expression.clone();
                adapt_expression(&mut expression);
                elements.push(ast::PatternElement::Placeable { expression });
            }
        }
    }
    pattern.elements = elements;
}

fn adapt_expression(expression: &mut ast::Expression<String>) {
    match expression {
        ast::Expression::SelectExpression { selector, variants } => {
            for variant in variants {
                adapt_pattern(&mut variant.value);
            }
        }
        ast::Expression::InlineExpression(_) => {}
    }
}

fn adapt_ast(ast: &mut ast::Resource<String>) {
    for entry in &mut ast.body {
        match entry {
            ast::Entry::Comment(comment)
            | ast::Entry::GroupComment(comment)
            | ast::Entry::ResourceComment(comment) => {
                adapt_comment(comment);
            }
            ast::Entry::Message(msg) => {
                if let Some(pattern) = &mut msg.value {
                    adapt_pattern(pattern);
                }
                for attr in &mut msg.attributes {
                    adapt_pattern(&mut attr.value);
                }
                if let Some(comment) = &mut msg.comment {
                    adapt_comment(comment);
                }
            }
            ast::Entry::Term(term) => {
                adapt_pattern(&mut term.value);
                if let Some(comment) = &mut term.comment {
                    adapt_comment(comment);
                }
            }
            _ => {}
        }
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
        // if BLACKLIST.contains(&path) {
        //     continue;
        // }

        let reference_path = path.replace(".ftl", ".json");
        let reference_file = read_file(&reference_path, true).unwrap();
        let ftl_file = read_file(&path, false).unwrap();

        println!("Parsing: {:#?}", path);
        let target_ast = match Parser::new(ftl_file).parse() {
            Ok(res) => res,
            Err((res, _errors)) => res,
        };

        // let target_json = json::serialize(&target_ast).unwrap();
        let mut ref_ast: ast::Resource<String> =
            serde_json::from_str(reference_file.as_str()).unwrap();
        adapt_ast(&mut ref_ast);
        // let s = r#"
        // {
        //                 "type": "Pattern",
        //                 "elements": [
        //                     {
        //                         "type": "TextElement",
        //                         "value": "which is an attribute\nContinued"
        //                     }
        //                 ]
        // }
        // "#;
        // let mut ref_ast: ast::Pattern<String> = serde_json::from_str(s).unwrap();
        // adapt_pattern(&mut ref_ast);
        // println!("{:#?}", ref_ast);

        // compare_jsons(&target_json, &reference_file);
        assert_eq!(target_ast.body.len(), ref_ast.body.len());
        for (entry, ref_entry) in target_ast.body.iter().zip(ref_ast.body.iter()) {
            assert_eq!(entry, ref_entry);
        }
    }
}

// #[test]
// fn parse_fixtures() {
//     for entry in glob("./tests/fixtures/*.ftl").expect("Failed to read glob pattern") {
//         let p = entry.expect("Error while getting an entry");
//         let path = p.to_str().expect("Can't print path");

//         println!("Attempting to parse file: {}", path);

//         let string = read_file(path, false).expect("Failed to read");

//         let _ = Parser::new(string.as_str()).parse();
//     }
// }

// #[test]
// fn parse_bench_fixtures() {
//     for entry in glob("./benches/*.ftl").expect("Failed to read glob pattern") {
//         let p = entry.expect("Error while getting an entry");
//         let path = p.to_str().expect("Can't print path");
//         let file_name = p.file_name().unwrap().to_str().unwrap();

//         let reference_path = format!(
//             "./tests/fixtures/benches/{}",
//             file_name.replace(".ftl", ".json")
//         );
//         let reference_file = read_file(&reference_path, true).unwrap();
//         let ftl_file = read_file(&path, false).unwrap();

//         println!("Parsing: {:#?}", path);
//         let target_ast = match parse(&ftl_file) {
//             Ok(res) => res,
//             Err((res, _errors)) => res,
//         };

//         let target_json = json::serialize(&target_ast).unwrap();

//         compare_jsons(&target_json, &reference_file);
//     }

//     let contexts = &["browser", "preferences"];

//     for context in contexts {
//         for entry in glob(&format!("./benches/contexts/{}/*.ftl", context))
//             .expect("Failed to read glob pattern")
//         {
//             let p = entry.expect("Error while getting an entry");
//             let path = p.to_str().expect("Can't print path");
//             let file_name = p.file_name().unwrap().to_str().unwrap();

//             let reference_path = format!(
//                 "./tests/fixtures/benches/contexts/{}/{}",
//                 context,
//                 file_name.replace(".ftl", ".json")
//             );
//             let reference_file = read_file(&reference_path, true).unwrap();
//             let ftl_file = read_file(&path, false).unwrap();

//             println!("Parsing: {:#?}", path);
//             let target_ast = match parse(&ftl_file) {
//                 Ok(res) => res,
//                 Err((res, _errors)) => res,
//             };

//             let target_json = json::serialize(&target_ast).unwrap();

//             compare_jsons(&target_json, &reference_file);
//         }
//     }
// }
