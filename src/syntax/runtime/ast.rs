use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Resource(
    pub HashMap<String, Value>
);

#[derive(Debug, PartialEq)]
pub enum Value {
    Simple(String),
}
