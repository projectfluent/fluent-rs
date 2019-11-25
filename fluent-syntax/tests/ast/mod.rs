use fluent_syntax::parser2::ast;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::error::Error;
use std::ops::Range;

pub fn serialize<'s>(res: &'s ast::Resource) -> Result<String, Box<dyn Error>> {
    #[derive(Serialize)]
    struct Helper<'s>(#[serde(with = "ResourceDef")] &'s ast::Resource);
    Ok(serde_json::to_string(&Helper(res)).unwrap())
}

pub fn _serialize_to_pretty_json<'s>(res: &'s ast::Resource) -> Result<String, Box<dyn Error>> {
    #[derive(Serialize)]
    struct Helper<'s>(#[serde(with = "ResourceDef")] &'s ast::Resource);

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
pub struct ResourceDef {
    #[serde(serialize_with = "serialize_resource_body")]
    pub body: Box<[ast::ResourceEntry]>,
}

static mut SOURCE: Option<String> = None;

pub fn set_source(source: String) {
    unsafe {
        SOURCE = Some(source);
    }
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
        Message(&'s ast::Message),
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
                ast::Entry::Comment(ref comment) => match comment.comment_type {
                    ast::CommentType::Regular => EntryHelper::Comment {
                        content: "Foo".to_string(),
                    },
                    _ => panic!(),
                },
            },
        };
        seq.serialize_element(&entry)?;
    }
    seq.end()
}

#[derive(Serialize)]
#[serde(remote = "ast::ResourceEntry")]
pub enum ResourceEntryDef {
    #[serde(with = "EntryDef")]
    Entry(ast::Entry),
}

#[derive(Serialize)]
#[serde(remote = "ast::Entry")]
#[serde(tag = "type")]
pub enum EntryDef {
    #[serde(with = "MessageDef")]
    Message(ast::Message),
    #[serde(with = "CommentDef")]
    Comment(ast::Comment),
}

#[derive(Serialize)]
#[serde(remote = "ast::Message")]
pub struct MessageDef {
    #[serde(with = "IdentifierDef")]
    pub id: ast::Identifier,
    #[serde(serialize_with = "serialize_pattern_option")]
    pub value: Option<ast::Pattern>,
    #[serde(serialize_with = "serialize_attribute_list")]
    pub attributes: Box<[ast::Attribute]>,
    #[serde(serialize_with = "serialize_comment_option")]
    pub comment: Option<ast::Comment>,
}

fn serialize_attribute_list<S>(v: &Box<[ast::Attribute]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'s>(#[serde(with = "AttributeDef")] &'s ast::Attribute);
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
    struct Helper<'s>(#[serde(with = "PatternDef")] &'s ast::Pattern);
    v.as_ref().map(Helper).serialize(serializer)
}

fn serialize_range_option<S>(v: &Option<Range<usize>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(range) = v {
        let source = unsafe { SOURCE.as_ref().unwrap() };

        let result = &source[range.start..range.end];
        serializer.serialize_str(result)
    } else {
        panic!()
    }
}

fn serialize_comment_option<S>(v: &Option<ast::Comment>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(comment) = v {
        let source = unsafe { SOURCE.as_ref().unwrap() };

        let mut result = String::new();

        for elem in comment.content.iter() {
            result.push_str(&source[elem.start..elem.end]);
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
pub struct PatternDef {
    #[serde(serialize_with = "serialize_pattern_element_list")]
    pub elements: Box<[ast::PatternElement]>,
}

fn serialize_pattern_element_list<S>(
    v: &Box<[ast::PatternElement]>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'s>(#[serde(with = "PatternElementDef")] &'s ast::PatternElement);
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    for e in v.iter() {
        seq.serialize_element(&Helper(e))?;
    }
    seq.end()
}

#[derive(Serialize)]
#[serde(remote = "ast::PatternElement")]
#[serde(tag = "type")]
pub enum PatternElementDef {
    #[serde(serialize_with = "serialize_text_element")]
    TextElement(Range<usize>),
}

fn serialize_text_element<S>(s: &Range<usize>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let source = unsafe { SOURCE.as_ref().unwrap() };
    let result = &source[s.start..s.end];

    let mut map = serializer.serialize_map(Some(2))?;
    map.serialize_entry("type", "TextElement")?;
    map.serialize_entry("value", result)?;
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
pub struct IdentifierDef {
    #[serde(serialize_with = "serialize_range")]
    pub name: Range<usize>,
}

fn serialize_range<S>(v: &Range<usize>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let source = unsafe { SOURCE.as_ref().unwrap() };
    serializer.serialize_str(&source[v.start..v.end])
}

#[derive(Serialize)]
#[serde(remote = "ast::CommentType")]
pub enum CommentTypeDef {
    Regular,
    Group,
    Resource,
}

#[derive(Serialize)]
#[serde(remote = "ast::Comment")]
pub struct CommentDef {
    #[serde(with = "CommentTypeDef")]
    pub comment_type: ast::CommentType,
    #[serde(serialize_with = "serialize_range_slice")]
    pub content: Box<[Range<usize>]>,
}

fn serialize_range_slice<S>(v: &Box<[Range<usize>]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let source = unsafe { SOURCE.as_ref().unwrap() };

    let mut result = String::new();

    for elem in v.iter() {
        result.push_str(&source[elem.start..elem.end]);
    }
    serializer.serialize_str(&result)
}
