#[derive(Debug, PartialEq)]
pub struct Resource<'ast> {
    pub body: Box<[ResourceEntry<'ast>]>,
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
    pub attributes: Box<[Attribute<'ast>]>,
    pub comment: Option<Comment<'ast>>,
}

#[derive(Debug, PartialEq)]
pub struct Term<'ast> {
    pub id: Identifier<'ast>,
    pub value: Pattern<'ast>,
    pub attributes: Box<[Attribute<'ast>]>,
    pub comment: Option<Comment<'ast>>,
}

#[derive(Debug, PartialEq)]
pub struct Pattern<'ast> {
    pub elements: Box<[PatternElement<'ast>]>,
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
pub enum CommentType {
    Regular,
    Group,
    Resource,
}

#[derive(Debug, PartialEq)]
pub struct Comment<'ast> {
    pub comment_type: CommentType,
    pub content: Box<[&'ast str]>,
}

#[derive(Debug, PartialEq)]
pub enum Expression<'ast> {
    InlineExpression(InlineExpression<'ast>),
}

#[derive(Debug, PartialEq)]
pub enum InlineExpression<'ast> {
    StringLiteral {
        value: &'ast str,
    },
    NumberLiteral {
        value: &'ast str,
    },
    MessageReference {
        id: Identifier<'ast>,
        attribute: Option<Identifier<'ast>>,
    },
    TermReference {
        id: Identifier<'ast>,
        attribute: Option<Identifier<'ast>>,
        arguments: Option<CallArguments<'ast>>,
    },
}

#[derive(Debug, PartialEq)]
pub struct CallArguments<'ast> {
    pub positional: Box<[InlineExpression<'ast>]>,
    pub named: Box<[NamedArgument<'ast>]>,
}

#[derive(Debug, PartialEq)]
pub struct NamedArgument<'ast> {
    pub name: Identifier<'ast>,
    pub value: InlineExpression<'ast>,
}
