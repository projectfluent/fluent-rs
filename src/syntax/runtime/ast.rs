#[derive(Debug, PartialEq)]
pub struct Resource(pub Vec<Entry>);

#[derive(Debug, PartialEq)]
pub enum Entry {
    Message(Message),
}

#[derive(Debug, PartialEq)]
pub struct Identifier(String);

#[derive(Debug, PartialEq)]
pub struct Message {
    pub id: String,
    pub value: Option<Pattern>,
    pub traits: Option<Vec<Member>>,
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub elements: Vec<PatternElement>,
}

#[derive(Debug, PartialEq)]
pub enum PatternElement {
    Text(String),
    Placeable(Vec<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct Member {
    pub key: String,
    pub value: Pattern,
    pub default: bool,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    EntityReference(Identifier),
}
