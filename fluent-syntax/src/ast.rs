pub use crate::arc_str::ArcStr;

#[derive(Debug, PartialEq)]
pub struct Resource {
    pub body: Vec<ResourceEntry>,
}

#[derive(Debug, PartialEq)]
pub enum ResourceEntry {
    Entry(Entry),
    Junk(ArcStr),
}

#[derive(Debug, PartialEq)]
pub enum Entry {
    Message(Message),
    Term(Term),
    Comment(Comment),
}

#[derive(Debug, PartialEq)]
pub struct Message {
    pub id: Identifier,
    pub value: Option<Pattern>,
    pub attributes: Vec<Attribute>,
    pub comment: Option<Comment>,
}

#[derive(Debug, PartialEq)]
pub struct Term {
    pub id: Identifier,
    pub value: Pattern,
    pub attributes: Vec<Attribute>,
    pub comment: Option<Comment>,
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub elements: Vec<PatternElement>,
}

#[derive(Debug, PartialEq)]
pub enum PatternElement {
    TextElement(ArcStr),
    Placeable(Expression),
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub id: Identifier,
    pub value: Pattern,
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: ArcStr,
}

#[derive(Debug, PartialEq)]
pub struct Variant {
    pub key: VariantKey,
    pub value: Pattern,
    pub default: bool,
}

#[derive(Debug, PartialEq)]
pub enum VariantKey {
    Identifier { name: ArcStr },
    NumberLiteral { value: ArcStr },
}

#[derive(Debug, PartialEq)]
pub enum Comment {
    Comment { content: Vec<ArcStr> },
    GroupComment { content: Vec<ArcStr> },
    ResourceComment { content: Vec<ArcStr> },
}

#[derive(Debug, PartialEq)]
pub enum InlineExpression {
    StringLiteral {
        value: ArcStr,
    },
    NumberLiteral {
        value: ArcStr,
    },
    FunctionReference {
        id: Identifier,
        arguments: Option<CallArguments>,
    },
    MessageReference {
        id: Identifier,
        attribute: Option<Identifier>,
    },
    TermReference {
        id: Identifier,
        attribute: Option<Identifier>,
        arguments: Option<CallArguments>,
    },
    VariableReference {
        id: Identifier,
    },
    Placeable {
        expression: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub struct CallArguments {
    pub positional: Vec<InlineExpression>,
    pub named: Vec<NamedArgument>,
}

#[derive(Debug, PartialEq)]
pub struct NamedArgument {
    pub name: Identifier,
    pub value: InlineExpression,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    InlineExpression(InlineExpression),
    SelectExpression {
        selector: InlineExpression,
        variants: Vec<Variant>,
    },
}
