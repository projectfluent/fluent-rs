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
    pub elements: Vec<PatternElement>,
    pub quoted: bool,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Selector(SelectorExpression),
    SelectExpression {
        exp: Option<SelectorExpression>,
        variants: Vec<Variant>
    },
}

#[derive(Debug, PartialEq)]
pub enum SelectorExpression {
    Pattern(Pattern),
    MessageReference(String),
    ExternalArgument(String),
    CallExpression {
        callee: Builtin,
        args: Vec<SelectorExpression>,
    },
    VariantExpression {
        id: Identifier,
        key: VariantKey
    },
    AttributeExpression {
        id: Identifier,
        attr: Identifier
    },
    KeyValueArgument {
        id: Identifier,
        val: ArgValue
    },
    Number(Number),
    Placeable(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub id: Identifier,
    pub value: Pattern
}

#[derive(Debug, PartialEq)]
pub struct Variant {
    pub key: VariantKey,
    pub value: Pattern,
    pub default: bool,
}

#[derive(Debug, PartialEq)]
pub enum VariantKey {
    Key(Keyword),
    Number(Number),
}

#[derive(Debug, PartialEq)]
pub enum ArgValue {
    Number(Number),
    String(String)
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String
}

#[derive(Debug, PartialEq)]
pub struct Keyword {
    pub value: String
}

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub body: String,
}

#[derive(Debug, PartialEq)]
pub struct Section {
    pub key: Keyword,
    pub body: Vec<Entry>,
    pub comment: Option<Comment>,
}

#[derive(Debug, PartialEq)]
pub enum PatternElement {
    Text(String),
    Placeable(Expression),
}

#[derive(Debug, PartialEq)]
pub struct Number {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct Builtin {
    pub id: String
}

#[derive(Debug, PartialEq)]
pub struct JunkEntry {
    pub body: String,
}
