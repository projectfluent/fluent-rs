#[derive(Debug, PartialEq)]
pub struct Resource<'ast> {
    pub body: Vec<ResourceEntry<'ast>>,
}

#[derive(Debug, PartialEq)]
pub enum ResourceEntry<'ast> {
    Entry(Entry<'ast>),
    Junk(&'ast str),
}

#[derive(Debug, PartialEq)]
pub enum Entry<'ast> {
    Message(Message<'ast>),
    Term(Term<'ast>),
    Comment(Comment<'ast>),
}

#[derive(Debug, PartialEq)]
pub struct Message<'ast> {
    pub id: Identifier<'ast>,
    pub value: Option<Pattern<'ast>>,
    pub attributes: Vec<Attribute<'ast>>,
    pub comment: Option<Comment<'ast>>,
}

#[derive(Debug, PartialEq)]
pub struct Term<'ast> {
    pub id: Identifier<'ast>,
    pub value: Value<'ast>,
    pub attributes: Vec<Attribute<'ast>>,
    pub comment: Option<Comment<'ast>>,
}

#[derive(Debug, PartialEq)]
pub enum Value<'ast> {
    Pattern(Pattern<'ast>),
    VariantList { variants: Vec<Variant<'ast>> },
}

#[derive(Debug, PartialEq)]
pub struct Pattern<'ast> {
    pub elements: Vec<PatternElement<'ast>>,
}

#[derive(Debug, PartialEq)]
pub enum PatternElement<'ast> {
    TextElement(&'ast str),
    Placeable(Expression<'ast>),
}

#[derive(Debug, PartialEq)]
pub struct Attribute<'ast> {
    pub id: Identifier<'ast>,
    pub value: Pattern<'ast>,
}

#[derive(Debug, PartialEq)]
pub struct Identifier<'ast> {
    pub name: &'ast str,
}

#[derive(Debug, PartialEq)]
pub struct Variant<'ast> {
    pub key: VariantKey<'ast>,
    pub value: Pattern<'ast>,
    pub default: bool,
}

#[derive(Debug, PartialEq)]
pub enum VariantKey<'ast> {
    Identifier { name: &'ast str },
    NumberLiteral { value: &'ast str },
}

#[derive(Debug, PartialEq)]
pub enum Comment<'ast> {
    Comment { content: Vec<&'ast str> },
    GroupComment { content: Vec<&'ast str> },
    ResourceComment { content: Vec<&'ast str> },
}

#[derive(Debug, PartialEq)]
pub enum InlineExpression<'ast> {
    StringLiteral {
        raw: &'ast str,
    },
    NumberLiteral {
        value: &'ast str,
    },
    VariableReference {
        id: Identifier<'ast>,
    },
    CallExpression {
        callee: Box<InlineExpression<'ast>>,
        positional: Vec<InlineExpression<'ast>>,
        named: Vec<NamedArgument<'ast>>,
    },
    AttributeExpression {
        reference: Box<InlineExpression<'ast>>,
        name: Identifier<'ast>,
    },
    VariantExpression {
        reference: Box<InlineExpression<'ast>>,
        key: VariantKey<'ast>,
    },
    MessageReference {
        id: Identifier<'ast>,
    },
    TermReference {
        id: Identifier<'ast>,
    },
    FunctionReference {
        id: Identifier<'ast>,
    },
    Placeable {
        expression: Box<Expression<'ast>>,
    },
}

#[derive(Debug, PartialEq)]
pub struct NamedArgument<'ast> {
    pub name: Identifier<'ast>,
    pub value: InlineExpression<'ast>,
}

#[derive(Debug, PartialEq)]
pub enum Expression<'ast> {
    InlineExpression(InlineExpression<'ast>),
    SelectExpression {
        selector: InlineExpression<'ast>,
        variants: Vec<Variant<'ast>>,
    },
}
