use fluent_syntax::ast;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::error::Error;

pub fn serialize<'s>(res: &'s ast::Resource<'s>) -> Result<String, Box<dyn Error>> {
    #[derive(Serialize)]
    struct Helper<'s>(#[serde(with = "ResourceDef")] &'s ast::Resource<'s>);
    Ok(serde_json::to_string(&Helper(res)).unwrap())
}

pub fn _serialize_to_pretty_json<'s>(res: &'s ast::Resource<'s>) -> Result<String, Box<dyn Error>> {
    #[derive(Serialize)]
    struct Helper<'s>(#[serde(with = "ResourceDef")] &'s ast::Resource<'s>);

    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    Helper(res).serialize(&mut ser).unwrap();
    Ok(String::from_utf8(ser.into_inner()).unwrap())
}

#[derive(Serialize)]
#[serde(remote = "ast::Resource")]
#[serde(tag = "type")]
#[serde(rename = "Resource")]
pub struct ResourceDef<'s> {
    #[serde(serialize_with = "serialize_resource_body")]
    pub body: Box<[ast::ResourceEntry<'s>]>,
}

fn serialize_resource_body<S>(
    v: &Box<[ast::ResourceEntry]>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    #[serde(tag = "type")]
    enum EntryHelper<'s> {
        Junk {
            annotations: Vec<&'s str>,
            content: &'s str,
        },
        #[serde(with = "MessageDef")]
        Message(&'s ast::Message<'s>),
        #[serde(with = "TermDef")]
        Term(&'s ast::Term<'s>),
        Comment {
            content: String,
        },
        GroupComment {
            content: String,
        },
        ResourceComment {
            content: String,
        },
    }

    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    for e in v.iter() {
        let entry = match *e {
            ast::ResourceEntry::Entry(ref entry) => match entry {
                ast::Entry::Message(ref msg) => EntryHelper::Message(msg),
                ast::Entry::Term(ref term) => EntryHelper::Term(term),
                ast::Entry::Comment(ref comment) => {
                    let content = comment.content.join("\n");
                    match comment.comment_type {
                        ast::CommentType::Regular => EntryHelper::Comment { content },
                        ast::CommentType::Group => EntryHelper::GroupComment { content },
                        ast::CommentType::Resource => EntryHelper::ResourceComment { content },
                    }
                }
            },
            ast::ResourceEntry::Junk(ref junk) => EntryHelper::Junk {
                content: junk,
                annotations: vec![],
            },
        };
        seq.serialize_element(&entry)?;
    }
    seq.end()
}

#[derive(Serialize)]
#[serde(remote = "ast::Message")]
pub struct MessageDef<'s> {
    #[serde(with = "IdentifierDef")]
    pub id: ast::Identifier<'s>,
    #[serde(serialize_with = "serialize_pattern_option")]
    pub value: Option<ast::Pattern<'s>>,
    #[serde(serialize_with = "serialize_attribute_list")]
    pub attributes: Box<[ast::Attribute<'s>]>,
    #[serde(serialize_with = "serialize_comment_option")]
    pub comment: Option<ast::Comment<'s>>,
}

#[derive(Serialize)]
#[serde(remote = "ast::Term")]
pub struct TermDef<'s> {
    #[serde(with = "IdentifierDef")]
    pub id: ast::Identifier<'s>,
    #[serde(with = "PatternDef")]
    pub value: ast::Pattern<'s>,
    #[serde(serialize_with = "serialize_attribute_list")]
    pub attributes: Box<[ast::Attribute<'s>]>,
    #[serde(serialize_with = "serialize_comment_option")]
    pub comment: Option<ast::Comment<'s>>,
}

fn serialize_attribute_list<S>(v: &Box<[ast::Attribute]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'s>(#[serde(with = "AttributeDef")] &'s ast::Attribute<'s>);
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    for e in v.iter() {
        seq.serialize_element(&Helper(e))?;
    }
    seq.end()
}

fn serialize_pattern_option<S>(v: &Option<ast::Pattern>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'s>(#[serde(with = "PatternDef")] &'s ast::Pattern<'s>);
    v.as_ref().map(Helper).serialize(serializer)
}

fn serialize_comment_option<S>(v: &Option<ast::Comment>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(comment) = v {
        let content = comment.content.join("\n");

        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("type", "Comment")?;
        map.serialize_entry("content", &content)?;
        map.end()
    } else {
        serializer.serialize_none()
    }
}

#[derive(Serialize)]
#[serde(remote = "ast::Pattern")]
#[serde(tag = "type")]
#[serde(rename = "Pattern")]
pub struct PatternDef<'s> {
    #[serde(serialize_with = "serialize_pattern_element_list")]
    pub elements: Box<[ast::PatternElement<'s>]>,
}

fn serialize_pattern_element_list<S>(
    v: &Box<[ast::PatternElement]>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'s>(#[serde(with = "PatternElementDef")] &'s ast::PatternElement<'s>);
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    let mut buffer = String::new();
    for e in v.iter() {
        match e {
            ast::PatternElement::TextElement(e) => {
                buffer.push_str(e);
            }
            _ => {
                if !buffer.is_empty() {
                    seq.serialize_element(&Helper(&ast::PatternElement::TextElement(&buffer)))?;
                    buffer = String::new();
                }

                seq.serialize_element(&Helper(e))?;
            }
        }
    }
    if !buffer.is_empty() {
        seq.serialize_element(&Helper(&ast::PatternElement::TextElement(&buffer)))?;
    }

    seq.end()
}

#[derive(Serialize)]
#[serde(remote = "ast::PatternElement")]
#[serde(tag = "type")]
pub enum PatternElementDef<'s> {
    #[serde(serialize_with = "serialize_text_element")]
    TextElement(&'s str),
    #[serde(serialize_with = "serialize_placeable")]
    Placeable(ast::Expression<'s>),
}

fn serialize_text_element<'s, S>(s: &'s str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(2))?;
    map.serialize_entry("type", "TextElement")?;
    map.serialize_entry("value", s)?;
    map.end()
}

fn serialize_placeable<'s, S>(exp: &ast::Expression<'s>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(#[serde(with = "ExpressionDef")] &'ast ast::Expression<'ast>);
    let mut map = serializer.serialize_map(Some(2))?;
    map.serialize_entry("type", "Placeable")?;
    map.serialize_entry("expression", &Helper(exp))?;
    map.end()
}

#[derive(Serialize)]
#[serde(remote = "ast::Attribute")]
#[serde(tag = "type")]
#[serde(rename = "Attribute")]
pub struct AttributeDef<'s> {
    #[serde(with = "IdentifierDef")]
    pub id: ast::Identifier<'s>,
    #[serde(with = "PatternDef")]
    pub value: ast::Pattern<'s>,
}

#[derive(Serialize)]
#[serde(remote = "ast::Identifier")]
#[serde(tag = "type")]
#[serde(rename = "Identifier")]
pub struct IdentifierDef<'s> {
    pub name: &'s str,
}

#[derive(Serialize)]
#[serde(remote = "ast::Expression")]
#[serde(tag = "type")]
pub enum ExpressionDef<'ast> {
    #[serde(with = "InlineExpressionDef")]
    InlineExpression(ast::InlineExpression<'ast>),
}

#[derive(Serialize)]
#[serde(remote = "ast::InlineExpression")]
#[serde(tag = "type")]
pub enum InlineExpressionDef<'s> {
    StringLiteral {
        value: &'s str,
    },
    NumberLiteral {
        value: &'s str,
    },
    MessageReference {
        #[serde(with = "IdentifierDef")]
        id: ast::Identifier<'s>,
        #[serde(serialize_with = "serialize_identifier_option")]
        attribute: Option<ast::Identifier<'s>>,
    },
    TermReference {
        #[serde(with = "IdentifierDef")]
        id: ast::Identifier<'s>,
        #[serde(serialize_with = "serialize_identifier_option")]
        attribute: Option<ast::Identifier<'s>>,
        #[serde(serialize_with = "serialize_call_arguments_option")]
        arguments: Option<ast::CallArguments<'s>>,
    },
}

fn serialize_call_arguments_option<'se, S>(
    v: &Option<ast::CallArguments<'se>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(#[serde(with = "CallArgumentsDef")] &'ast ast::CallArguments<'ast>);
    v.as_ref().map(Helper).serialize(serializer)
}

fn serialize_identifier_option<'se, S>(
    v: &Option<ast::Identifier<'se>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(#[serde(with = "IdentifierDef")] &'ast ast::Identifier<'ast>);
    v.as_ref().map(Helper).serialize(serializer)
}

#[derive(Serialize)]
#[serde(remote = "ast::CallArguments")]
#[serde(tag = "type")]
#[serde(rename = "CallArguments")]
pub struct CallArgumentsDef<'ast> {
    #[serde(serialize_with = "serialize_inline_expressions")]
    pub positional: Box<[ast::InlineExpression<'ast>]>,
    #[serde(serialize_with = "serialize_named_arguments")]
    pub named: Box<[ast::NamedArgument<'ast>]>,
}

#[derive(Serialize)]
#[serde(remote = "ast::NamedArgument")]
#[serde(tag = "type")]
#[serde(rename = "NamedArgument")]
pub struct NamedArgumentDef<'ast> {
    #[serde(with = "IdentifierDef")]
    pub name: ast::Identifier<'ast>,
    #[serde(with = "InlineExpressionDef")]
    pub value: ast::InlineExpression<'ast>,
}

fn serialize_inline_expressions<'se, S>(
    v: &Box<[ast::InlineExpression<'se>]>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(#[serde(with = "InlineExpressionDef")] &'ast ast::InlineExpression<'ast>);
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    for e in v.iter() {
        seq.serialize_element(&Helper(e))?;
    }
    seq.end()
}

fn serialize_named_arguments<'se, S>(
    v: &Box<[ast::NamedArgument<'se>]>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(#[serde(with = "NamedArgumentDef")] &'ast ast::NamedArgument<'ast>);
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    for e in v.iter() {
        seq.serialize_element(&Helper(e))?;
    }
    seq.end()
}
