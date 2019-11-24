use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct Resource {
    pub body: Box<[ResourceEntry]>,
}

#[derive(Debug, PartialEq)]
pub enum ResourceEntry {
    Entry(Entry),
}

#[derive(Debug, PartialEq)]
pub enum Entry {
    Message(Message),
}

#[derive(Debug, PartialEq)]
pub struct Message {
    pub id: Identifier,
    pub value: Option<Range<usize>>,
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: Range<usize>,
}
