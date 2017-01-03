use std::collections::HashMap;

use super::syntax::ast;
use super::syntax::parse;
use super::syntax::parser::ParserError;

pub struct MessageContext {
    messages: HashMap<String, ast::Message>,
}

impl MessageContext {
    pub fn new() -> MessageContext {
        MessageContext { messages: HashMap::new() }
    }

    pub fn add_messages(&mut self, source: &str) -> Result<(), ParserError> {
        let res = parse(source)?;

        for entry in res.body {
            match entry {
                ast::Entry::Message(msg @ ast::Message { .. }) => {
                    self.messages.insert(msg.id.name.clone(), msg);
                }
            }
        }

        Ok(())
    }

    pub fn get_message(&self, id: &str) -> Option<&ast::Message> {
        self.messages.get(id)
    }

    pub fn format(&self, msg: &ast::Message) -> Option<String> {
        msg.value.as_ref().and_then(|pattern| self.eval_pattern(pattern))
    }

    fn eval_expr(&self, expr: &ast::Expression) -> Option<String> {
        match expr {
            &ast::Expression::EntityReference { id: ast::Identifier { ref name } } => {
                self.messages
                    .get(name)
                    .and_then(|msg| self.format(msg))
            }
            &ast::Expression::ExternalArgument { id: ast::Identifier { ref name } } => {
                Some(format!("${}", name))
            }
            _ => unimplemented!(),
        }
    }

    fn eval_pattern(&self, pat: &ast::Pattern) -> Option<String> {
        let &ast::Pattern { ref elements, .. } = pat;
        let val = elements.iter()
            .map(|elem| {
                match elem {
                    &ast::PatternElement::Text(ref val) => val.clone(),
                    &ast::PatternElement::Placeable { ref expressions } => {
                        expressions.iter()
                            .map(|expr| self.eval_expr(expr).unwrap_or(String::from("___")))
                            .collect::<Vec<String>>()
                            .join(", ")
                    }
                }
            })
            .collect::<Vec<String>>()
            .join("");
        Some(val)
    }
}
