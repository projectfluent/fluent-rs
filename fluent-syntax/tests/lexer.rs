use fluent_syntax::parser2::lexer;

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
