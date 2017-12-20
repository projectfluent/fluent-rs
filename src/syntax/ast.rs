#[derive(Debug, PartialEq)]
pub struct Resource {
    pub body: Vec<Entry>,
}

#[derive(Debug, PartialEq)]
pub enum Entry {
    Message(Message),
    Comment(Comment),
    Junk { content: String },
}

#[derive(Debug, PartialEq)]
pub struct Message {
    pub id: Identifier,
    pub value: Option<Pattern>,
    pub attributes: Option<Vec<Attribute>>,
    pub comment: Option<Comment>,
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub elements: Vec<PatternElement>,
}

#[derive(Debug, PartialEq)]
pub enum PatternElement {
    TextElement(String),
    Placeable(Placeable),
}

#[derive(Debug, PartialEq)]
pub struct Placeable {
    pub expression: Expression,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    StringExpression {
        value: String,
    },
    NumberExpression {
        value: Number,
    },
    MessageReference {
        id: Identifier,
    },
    ExternalArgument {
        id: Identifier,
    },
    SelectExpression {
        expression: Option<Box<Expression>>,
        variants: Vec<Variant>,
    },
    AttributeExpression {
        id: Identifier,
        name: Identifier,
    },
    VariantExpression {
        id: Identifier,
        key: VarKey,
    },
    CallExpression {
        callee: Function,
        args: Vec<Argument>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub id: Identifier,
    pub value: Pattern,
}

#[derive(Debug, PartialEq)]
pub struct Variant {
    pub key: VarKey,
    pub value: Pattern,
    pub default: bool,
}

#[derive(Debug, PartialEq)]
pub enum VarKey {
    VariantName(VariantName),
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
pub struct Number {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct VariantName {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum Comment {
    Comment { content: String },
    GroupComment { content: String },
    ResourceComment { content: String },
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
}
