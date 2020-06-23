#[derive(Debug, PartialEq)]
pub struct Resource<S> {
    pub body: Vec<ResourceEntry<S>>,
}

#[derive(Debug, PartialEq)]
pub enum ResourceEntry<S> {
    Entry(Entry<S>),
    Junk(S),
}

#[derive(Debug, PartialEq)]
pub enum Entry<S> {
    Message(Message<S>),
    Term(Term<S>),
    Comment(Comment<S>),
}

#[derive(Debug, PartialEq)]
pub struct Message<S> {
    pub id: Identifier<S>,
    pub value: Option<Pattern<S>>,
    pub attributes: Vec<Attribute<S>>,
    pub comment: Option<Comment<S>>,
}

#[derive(Debug, PartialEq)]
pub struct Term<S> {
    pub id: Identifier<S>,
    pub value: Pattern<S>,
    pub attributes: Vec<Attribute<S>>,
    pub comment: Option<Comment<S>>,
}

#[derive(Debug, PartialEq)]
pub struct Pattern<S> {
    pub elements: Vec<PatternElement<S>>,
}

#[derive(Debug, PartialEq)]
pub enum PatternElement<S> {
    TextElement(S),
    Placeable(Expression<S>),
}

#[derive(Debug, PartialEq)]
pub struct Attribute<S> {
    pub id: Identifier<S>,
    pub value: Pattern<S>,
}

#[derive(Debug, PartialEq)]
pub struct Identifier<S> {
    pub name: S,
}

#[derive(Debug, PartialEq)]
pub struct Variant<S> {
    pub key: VariantKey<S>,
    pub value: Pattern<S>,
    pub default: bool,
}

#[derive(Debug, PartialEq)]
pub enum VariantKey<S> {
    Identifier { name: S },
    NumberLiteral { value: S },
}

#[derive(Debug, PartialEq)]
pub enum Comment<S> {
    Comment { content: Vec<S> },
    GroupComment { content: Vec<S> },
    ResourceComment { content: Vec<S> },
}

#[derive(Debug, PartialEq)]
pub enum InlineExpression<S> {
    StringLiteral {
        value: S,
    },
    NumberLiteral {
        value: S,
    },
    FunctionReference {
        id: Identifier<S>,
        arguments: Option<CallArguments<S>>,
    },
    MessageReference {
        id: Identifier<S>,
        attribute: Option<Identifier<S>>,
    },
    TermReference {
        id: Identifier<S>,
        attribute: Option<Identifier<S>>,
        arguments: Option<CallArguments<S>>,
    },
    VariableReference {
        id: Identifier<S>,
    },
    Placeable {
        expression: Box<Expression<S>>,
    },
}

#[derive(Debug, PartialEq)]
pub struct CallArguments<S> {
    pub positional: Vec<InlineExpression<S>>,
    pub named: Vec<NamedArgument<S>>,
}

#[derive(Debug, PartialEq)]
pub struct NamedArgument<S> {
    pub name: Identifier<S>,
    pub value: InlineExpression<S>,
}

#[derive(Debug, PartialEq)]
pub enum Expression<S> {
    InlineExpression(InlineExpression<S>),
    SelectExpression {
        selector: InlineExpression<S>,
        variants: Vec<Variant<S>>,
    },
}
