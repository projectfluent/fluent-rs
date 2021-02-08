use fluent_syntax::{ast, parser::Slice};

#[derive(Debug, PartialEq)]
pub struct FluentAttribute<'m, S> {
    pub id: &'m S,
    pub value: &'m ast::Pattern<S>,
}

impl<'m, S> From<&'m ast::Attribute<S>> for FluentAttribute<'m, S> {
    fn from(attr: &'m ast::Attribute<S>) -> Self {
        FluentAttribute {
            id: &attr.id.name,
            value: &attr.value,
        }
    }
}
/// A single localization unit composed of an identifier,
/// value, and attributes.
#[derive(Debug, PartialEq)]
pub struct FluentMessage<'m, S> {
    pub value: Option<&'m ast::Pattern<S>>,
    pub attributes: Vec<FluentAttribute<'m, S>>,
}

impl<'m, S> FluentMessage<'m, S> {
    pub fn get_attribute(&self, key: &str) -> Option<&FluentAttribute<S>>
    where
        S: Slice<'m>,
    {
        self.attributes.iter().find(|attr| attr.id.as_ref() == key)
    }
}

impl<'m, S> From<&'m ast::Message<S>> for FluentMessage<'m, S> {
    fn from(msg: &'m ast::Message<S>) -> Self {
        FluentMessage {
            value: msg.value.as_ref(),
            attributes: msg.attributes.iter().map(Into::into).collect(),
        }
    }
}
