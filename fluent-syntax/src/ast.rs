#[derive(Debug, PartialEq)]
pub struct Resource<'ast> {
    pub body: Box<[ResourceEntry<'ast>]>,
}

#[derive(Debug, PartialEq)]
pub enum ResourceEntry<'ast> {
    Entry(Entry<'ast>),
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
