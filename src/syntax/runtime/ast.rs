extern crate serde;
extern crate serde_json;

use self::serde::ser::{Serialize, Serializer};
use self::serde::de::{Deserialize, Deserializer, Visitor, MapVisitor, SeqVisitor, Error};
use self::serde::de::value::{ValueDeserializer, SeqVisitorDeserializer, MapVisitorDeserializer};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource(pub Vec<Entry>);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Identifier(String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Entry {
    Message(Message),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub value: Option<Pattern>,
    pub traits: Option<Vec<Member>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Pattern {
    pub elements: Vec<PatternElement>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PatternElement {
    Text(String),
    Placeable(Vec<Expression>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub key: String,
    pub value: Pattern,
    pub default: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    EntityReference(Identifier),
}
