extern crate serde;
extern crate serde_json;

use super::ast;

use self::serde::ser::{Serialize, Serializer};

impl Serialize for ast::Resource {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
        {
            let mut state = serializer.serialize_seq(Some(self.0.len()))?;
            for e in &self.0 {
                serializer.serialize_seq_elt(&mut state, e)?;
            }
            serializer.serialize_seq_end(state)
        }
}

impl Serialize for ast::Entry {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
        {
            match self {
                &ast::Entry::Message(ref msg) => msg.serialize(serializer)
            }
        }
}

impl Serialize for ast::Identifier {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
        {
            serializer.serialize_str(&self.0)
        }
}

impl Serialize for ast::Message {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
        {
            let mut state = serializer.serialize_struct("Message", 3)?;
            serializer.serialize_struct_elt(&mut state, "id", &self.id)?;
            serializer.serialize_struct_elt(&mut state, "value", &self.value)?;
            serializer.serialize_struct_elt(&mut state, "traits", &self.traits)?;
            serializer.serialize_struct_end(state)
        }
}

impl Serialize for ast::Pattern {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
        {
            let mut state = serializer.serialize_seq(Some(self.elements.len()))?;
            for e in &self.elements {
                serializer.serialize_seq_elt(&mut state, e)?;
            }
            serializer.serialize_seq_end(state)
        }
}

impl Serialize for ast::PatternElement {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
        {
            match self {
                &ast::PatternElement::Text(ref t) => t.serialize(serializer),
            }
        }
}

impl Serialize for ast::Member {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
        {
            let mut state = serializer.serialize_struct("Member", 3)?;
            serializer.serialize_struct_elt(&mut state, "key", &self.key)?;
            serializer.serialize_struct_elt(&mut state, "value", &self.value)?;
            serializer.serialize_struct_elt(&mut state, "default", &self.default)?;
            serializer.serialize_struct_end(state)
        }
}

pub fn serialize_json(res: &ast::Resource) -> String {
    serde_json::to_string_pretty(res).unwrap()
}
