use fluent_syntax::parser::lexer;
use std::fmt::Write;

fn main() {
    let input = include_str!("../../benches/menubar.ftl");

    let lexer = lexer::Lexer::new(input.as_bytes());
    let tokens: Vec<lexer::Token> = lexer.collect();

    let mut result = String::new();
    write!(result, "{:#?}", tokens).unwrap();
    println!("{}", result);
}
