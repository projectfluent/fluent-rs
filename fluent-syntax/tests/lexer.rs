use fluent_syntax::parser::lexer;
use glob::glob;
use std::fs::File;
use std::io;
use std::io::prelude::*;

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
fn lex_simple() {
    let input = include_str!("../benches/simple.ftl");

    let output = include_str!("./lexer/fixtures/simple.json").trim();

    let lexer = lexer::Lexer::new(input.as_bytes());
    let tokens: Vec<lexer::Token> = lexer.collect();

    let result = format!("{:#?}", tokens);
    assert_eq!(result, output);
}

#[test]
fn lex_menubar() {
    let input = include_str!("../benches/menubar.ftl");

    let output = include_str!("./lexer/fixtures/menubar.json").trim();

    let lexer = lexer::Lexer::new(input.as_bytes());
    let tokens: Vec<lexer::Token> = lexer.collect();

    let result = format!("{:#?}", tokens);
    assert_eq!(result, output);
}

#[test]
fn lex_reference() {
    for entry in
        glob("./tests/lexer/fixtures/reference/*.txt").expect("Failed to read glob pattern")
    {
        let p = entry.expect("Error while getting an entry");
        let name = p
            .file_name()
            .expect("File name doesn't exist")
            .to_str()
            .expect("Failed to extract a str.");
        let id = name.trim_end_matches(".txt");

        let input_path = format!("./tests/fixtures/{}.ftl", id);
        let input = read_file(&input_path, false).expect("Failed to read file.");
        let output = read_file(&p.to_str().expect("Failed to extract a str."), true)
            .expect("Failed to read file.");

        let lexer = lexer::Lexer::new(input.as_bytes());
        let tokens: Vec<lexer::Token> = lexer.collect();

        let result = format!("{:#?}", tokens);
        assert_eq!(result, output);
    }
}
