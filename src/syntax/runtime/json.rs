extern crate serde;
extern crate serde_json;

use super::super::ast;

use self::serde::ser::{Serialize, Serializer};

impl Serialize for ast::Resource {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_map(Some(self.0.len()))?;
        for e in &self.0 {
            match e {
                &ast::Entry::Message(ref msg @ ast::Message { .. }) => {
                    let id = msg.id.clone();
                    serializer.serialize_map_key(&mut state, id)?;
                    serializer.serialize_map_value(&mut state, e)?;
                }
            }


        }
        serializer.serialize_map_end(state)
    }
}

impl Serialize for ast::Entry {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        match self {
            &ast::Entry::Message(ref msg) => msg.serialize(serializer),
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
        if let Some(ref val) = self.value {
            if val.elements.len() == 1 && self.traits.is_none() {
                match val.elements.first() {
                    Some(&ast::PatternElement::Text(ref t)) => {
                        return serializer.serialize_str(&t);
                    }
                    _ => {}
                }
            }
        }

        let mut state = serializer.serialize_map(Some(2))?;
        serializer.serialize_map_key(&mut state, "value")?;
        serializer.serialize_map_value(&mut state, &self.value)?;
        serializer.serialize_map_key(&mut state, "traits")?;
        serializer.serialize_map_value(&mut state, &self.traits)?;
        serializer.serialize_map_end(state)
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
        let mut state = serializer.serialize_map(Some(3))?;
        serializer.serialize_map_key(&mut state, "key")?;
        serializer.serialize_map_value(&mut state, &self.key)?;
        serializer.serialize_map_key(&mut state, "value")?;
        serializer.serialize_map_value(&mut state, &self.value)?;
        serializer.serialize_map_key(&mut state, "default")?;
        serializer.serialize_map_value(&mut state, &self.default)?;
        serializer.serialize_map_end(state)
    }
}

pub fn serialize_json(res: &ast::Resource) -> String {
    serde_json::to_string_pretty(res).unwrap()
}
