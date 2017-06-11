use std::collections::HashMap;

use super::syntax::ast;
use super::syntax::parse;

pub struct MessageContext {
    messages: HashMap<String, ast::Entry>,
}

impl MessageContext {
    pub fn new() -> MessageContext {
        MessageContext { messages: HashMap::new() }
    }

    pub fn add_messages(&mut self, source: &str) {
        let res = parse(source).unwrap_or_else(|x| x.0);

        for entry in res.body {
            let id = match entry {
                ast::Entry::Message { ref id, .. } => id.name.clone(),
                _ => continue,
            };

            match entry {
                ast::Entry::Message { .. } => {
                    self.messages.insert(id, entry);
                }
                _ => continue,
            };
        }
    }

    pub fn get_message(&self, id: &str) -> Option<&ast::Entry> {
        self.messages.get(id)
    }

    pub fn format(&self, entry: &ast::Entry) -> Option<String> {
        match entry {
            &ast::Entry::Message { ref value, .. } => {
                value
                    .as_ref()
                    .and_then(|pattern| self.eval_pattern(pattern))
            }
            _ => unimplemented!(),
        }
    }

    fn eval_expr(&self, expr: &ast::Expression) -> Option<String> {
        match expr {
            &ast::Expression::StringExpression(ref val) => Some(val.clone()),
            &ast::Expression::MessageReference { ref id } => {
                self.messages.get(id).and_then(|msg| self.format(msg))
            }
            &ast::Expression::ExternalArgument { ref id } => Some(format!("${}", id)),
            _ => unimplemented!(),
        }
    }

    fn eval_pattern_elem(&self, expr: &ast::PatternElement) -> Option<String> {
        match expr {
            &ast::PatternElement::TextElement(ref val) => Some(val.clone()),
            &ast::PatternElement::Expression(ref exp) => self.eval_expr(exp),
        }
    }

    fn eval_pattern(&self, pat: &ast::Pattern) -> Option<String> {
        let &ast::Pattern { ref elements, .. } = pat;
        let val = elements
            .iter()
            .map(|elem| self.eval_pattern_elem(elem).unwrap_or(String::from("___")))
            .collect::<Vec<String>>()
            .join("");
        Some(val)
    }
}
