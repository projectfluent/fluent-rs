#[derive(Debug, PartialEq)]
pub struct Resource {
    pub body: Vec<Entry>,
    pub comment: Option<Comment>,
}

#[derive(Debug, PartialEq)]
pub struct Message {
    pub id: Identifier,
    pub value: Option<Pattern>,
    pub attributes: Option<Vec<Attribute>>,
    pub tags: Option<Vec<Tag>>,
    pub comment: Option<Comment>,
}

#[derive(Debug, PartialEq)]
pub enum Entry {
    Message(Message),
    Section(Section),
    Comment(Comment),
    Junk(JunkEntry),
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub elements: Vec<Expression>,
    pub quoted: bool,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    String(String),
    Number(Number),
    MessageReference { id: String },
    ExternalArgument { id: String },
    SelectExpression {
        expression: Option<Box<Expression>>,
        variants: Vec<Variant>,
    },
    AttributeExpression { id: Identifier, name: Identifier },
    VariantExpression { id: Identifier, key: VarKey },
    CallExpression {
        callee: Function,
        args: Vec<Argument>,
    },
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub id: Identifier,
    pub value: Pattern,
}

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub name: Symbol,
}

#[derive(Debug, PartialEq)]
pub struct Variant {
    pub key: VarKey,
    pub value: Pattern,
    pub default: bool,
}

#[derive(Debug, PartialEq)]
pub enum VarKey {
    Symbol(Symbol),
    Number(Number),
}

#[derive(Debug, PartialEq)]
pub enum Argument {
    Expression(Expression),
    NamedArgument { name: Identifier, val: ArgValue },
}

#[derive(Debug, PartialEq)]
pub enum ArgValue {
    Number(Number),
    String(String),
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub body: String,
}

#[derive(Debug, PartialEq)]
pub struct Section {
    pub name: Symbol,
    pub comment: Option<Comment>,
}

#[derive(Debug, PartialEq)]
pub struct Number {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct JunkEntry {
    pub content: String,
}
