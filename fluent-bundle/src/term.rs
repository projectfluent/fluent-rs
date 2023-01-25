use fluent_syntax::ast;

use crate::message::FluentAttribute;

#[derive(Debug, PartialEq)]
pub struct FluentTerm<'t> {
    node: &'t ast::Term<&'t str>,
}

impl<'t> FluentTerm<'t> {
    pub fn value(&self) -> &'t ast::Pattern<&'t str> {
        &self.node.value
    }

    pub fn attributes(&self) -> impl Iterator<Item = FluentAttribute<'t>> {
        self.node.attributes.iter().map(Into::into)
    }

    pub fn get_attribute(&self, key: &str) -> Option<FluentAttribute<'t>> {
        self.node
            .attributes
            .iter()
            .find(|attr| attr.id.name == key)
            .map(Into::into)
    }
}

impl<'t> From<&'t ast::Term<&'t str>> for FluentTerm<'t> {
    fn from(term: &'t ast::Term<&'t str>) -> Self {
        FluentTerm { node: term }
    }
}
