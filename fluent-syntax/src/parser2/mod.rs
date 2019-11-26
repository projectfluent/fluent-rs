pub mod ast;
mod lexer;
mod parser;

pub use parser::Parser;

#[cfg(test)]
mod tests {
    use super::ast;
    use super::parser::Parser;

    #[test]
    fn parser2_test() {
        let input = include_str!("../../benches/simple.ftl");
        let parser = Parser::new(input.as_bytes());
        let ast = parser.parse();

        assert_eq!(ast.body.len(), 101);
        let (id, value) = match &ast.body[1] {
            ast::ResourceEntry::Entry(ast::Entry::Message(msg)) => {
                let pe = &msg.value.as_ref().unwrap().elements[0];
                let text = match pe {
                    ast::PatternElement::TextElement(r) => r,
                };
                (&msg.id.name, text)
            }
            _ => panic!(),
        };
        let id = &input[id.start..id.end];
        let value = &input[value.start..value.end];
        assert_eq!(id, "key0");
        assert_eq!(value, "Value 0");

        let (id, value) = match &ast.body[2] {
            ast::ResourceEntry::Entry(ast::Entry::Message(msg)) => {
                let pe = &msg.value.as_ref().unwrap().elements[0];
                let text = match pe {
                    ast::PatternElement::TextElement(r) => r,
                };
                (&msg.id.name, text)
            }
            _ => panic!(),
        };
        let id = &input[id.start..id.end];
        let value = &input[value.start..value.end];
        assert_eq!(id, "key1");
        assert_eq!(value, "Value 1");
    }

    #[test]
    fn parser2_test2() {
        let input = include_str!("../../benches/menubar.ftl");
        let parser = Parser::new(input.as_bytes());
        let ast = parser.parse();

        assert_eq!(ast.body.len(), 101);
        let (id, value) = match &ast.body[1] {
            ast::ResourceEntry::Entry(ast::Entry::Message(msg)) => {
                let pe = &msg.value.as_ref().unwrap().elements[0];
                let text = match pe {
                    ast::PatternElement::TextElement(r) => r,
                };
                (&msg.id.name, text)
            }
            _ => panic!(),
        };
        let id = &input[id.start..id.end];
        let value = &input[value.start..value.end];
        assert_eq!(id, "key0");
        assert_eq!(value, "Value 0");
    }
}
