use std::collections::HashMap;

use super::syntax::ast;
use super::syntax::parse;

pub struct MessageContext {
    messages: HashMap<String, ast::Message>,
}

impl MessageContext {
    pub fn new() -> MessageContext {
        MessageContext { messages: HashMap::new() }
    }

    pub fn add_messages(&mut self, source: &str) {
        let res = parse(source).unwrap_or_else(|x| x.0);

        for entry in res.body {
            match entry {
                ast::Entry::Message(msg @ ast::Message { .. }) => {
                    self.messages.insert(msg.id.name.clone(), msg);
                }
                _ => unimplemented!(),
            }
        }
    }

    pub fn get_message(&self, id: &str) -> Option<&ast::Message> {
        self.messages.get(id)
    }

    pub fn format(&self, msg: &ast::Message) -> Option<String> {
        msg.value.as_ref().and_then(|pattern| self.eval_pattern(pattern))
    }

    fn eval_expr(&self, expr: &ast::Expression) -> Option<String> {
        match expr {
            &ast::Expression::String(ref val) => Some(val.clone()),
            &ast::Expression::MessageReference { ref id } => {
                self.messages
                    .get(id)
                    .and_then(|msg| self.format(msg))
            }
            &ast::Expression::ExternalArgument { ref id } => Some(format!("${}", id)),
            _ => unimplemented!(),
        }
    }

    fn eval_pattern(&self, pat: &ast::Pattern) -> Option<String> {
        let &ast::Pattern { ref elements, .. } = pat;
        let val = elements.iter()
            .map(|elem| self.eval_expr(elem).unwrap_or(String::from("___")))
            .collect::<Vec<String>>()
            .join("");
        Some(val)
    }
}
