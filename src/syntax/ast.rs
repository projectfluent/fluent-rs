#[derive(Debug, PartialEq)]
pub struct Resource {
    pub body: Vec<Entry>,
    pub comment: Option<Comment>,
}

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub body: String,
}

#[derive(Debug, PartialEq)]
pub struct Section {
    pub key: Key,
    pub body: Vec<Entry>,
    pub comment: Option<Comment>,
}

#[derive(Debug, PartialEq)]
pub enum Entry {
    Message(Message),
    Comment(Comment),
    Section(Section),
    Junk(JunkEntry),
}

#[derive(Debug, PartialEq)]
pub struct Message {
    pub id: Identifier,
    pub value: Option<Pattern>,
    pub traits: Option<Vec<Member>>,
    pub comment: Option<Comment>,
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub elements: Vec<PatternElement>,
    pub quoted: bool,
}

#[derive(Debug, PartialEq)]
pub enum PatternElement {
    Text(String),
    Placeable { expressions: Vec<Expression> },
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Key {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Number {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct Keyword {
    pub ns: Option<Identifier>,
    pub name: Key,
}

#[derive(Debug, PartialEq)]
pub enum MemberKey {
    Keyword(Keyword),
    Number(Number),
}

#[derive(Debug, PartialEq)]
pub struct Member {
    pub key: MemberKey,
    pub value: Pattern,
    pub default: bool,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    MessageReference { id: Identifier },
    ExternalArgument { id: Identifier },
    CallExpression {
        callee: Identifier,
        args: Vec<Expression>,
    },
    SelectExpression {
        exp: Box<Expression>,
        vars: Vec<Member>,
    },
    KeyValueArgument {
        name: Identifier,
        val: Box<Expression>,
    },
    Number(Number),
    String(String),
    Member {
        obj: Box<Expression>,
        key: MemberKey,
    },
}

#[derive(Debug, PartialEq)]
pub struct JunkEntry {
    pub body: String,
}
