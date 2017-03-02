#[derive(Debug, PartialEq)]
pub struct Resource {
    pub body: Vec<Entry>,
    pub comment: Option<Comment>,
}

#[derive(Debug, PartialEq)]
pub enum Entry {
    Message {
        id: Identifier,
        value: Option<Pattern>,
        attributes: Option<Vec<Attribute>>,
        tags: Option<Vec<Tag>>,
        comment: Option<Comment>,
    },
    Section {
        name: Symbol,
        comment: Option<Comment>,
    },
    Comment(Comment),
    Junk {
        content: String,
    },
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub elements: Vec<PatternElement>,
    pub quoted: bool,
}

#[derive(Debug, PartialEq)]
pub enum PatternElement {
    TextElement(String),
    Expression(Expression)
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    StringExpression(String),
    NumberExpression(Number),
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
pub struct Function {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Number {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub content: String,
}
