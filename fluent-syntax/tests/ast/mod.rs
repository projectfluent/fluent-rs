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
                ast::Entry::Comment(ref comment) => match comment.comment_type {
                    ast::CommentType::Regular => EntryHelper::Comment {
                        content: "Foo".to_string(),
                    },
                    ast::CommentType::Group => EntryHelper::GroupComment {
                        content: "Foo".to_string(),
                    },
                    ast::CommentType::Resource => EntryHelper::ResourceComment {
                        content: "Foo".to_string(),
                    },
                },
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
        let mut result = String::new();

        for elem in comment.content.iter() {
            result.push_str(&elem);
        }

        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("type", "Comment")?;
        map.serialize_entry("content", &result)?;
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
    for e in v.iter() {
        seq.serialize_element(&Helper(e))?;
    }
    seq.end()
}

#[derive(Serialize)]
#[serde(remote = "ast::PatternElement")]
#[serde(tag = "type")]
pub enum PatternElementDef<'s> {
    #[serde(serialize_with = "serialize_text_element")]
    TextElement(&'s str),
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

#[derive(Serialize)]
#[serde(remote = "ast::Attribute")]
#[serde(tag = "type")]
#[serde(rename = "Attribute")]
pub struct AttributeDef {}

#[derive(Serialize)]
#[serde(remote = "ast::Identifier")]
#[serde(tag = "type")]
#[serde(rename = "Identifier")]
pub struct IdentifierDef<'s> {
    pub name: &'s str,
}
