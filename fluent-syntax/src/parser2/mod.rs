mod lexer;
mod parser;
pub mod ast;

pub use parser::Parser;

#[cfg(test)]
mod tests {
    use super::parser::Parser;
    use super::ast;

    #[test]
    fn parser2_test() {
        let input = include_str!("../../benches/simple.ftl");
        // let input = r#"key0 = Value 0
// key1 = Value 1"#;
        let parser = Parser::new(input.as_bytes());
        let ast = parser.parse();

        assert_eq!(ast.body.len(), 100);
        let (id, value) = match &ast.body[0] {
            ast::ResourceEntry::Entry(ast::Entry::Message(msg)) => {
                (&msg.id.name, msg.value.as_ref().unwrap())
            },
        };
        let id = &input[id.start .. id.end];
        let value = &input[value.start .. value.end];
        assert_eq!(id, "key0");
        assert_eq!(value, "Value 0");

        let (id, value) = match &ast.body[1] {
            ast::ResourceEntry::Entry(ast::Entry::Message(msg)) => {
                (&msg.id.name, msg.value.as_ref().unwrap())
            },
        };
        let id = &input[id.start .. id.end];
        let value = &input[value.start .. value.end];
        assert_eq!(id, "key1");
        assert_eq!(value, "Value 1");
    }
}
