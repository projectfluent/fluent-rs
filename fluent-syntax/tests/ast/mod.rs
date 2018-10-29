mod helper;

use fluent_syntax::ast;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use serde_derive::Serialize;
use std::error::Error;

pub fn serialize<'s>(res: &'s ast::Resource) -> Result<String, Box<Error>> {
    #[derive(Serialize)]
    struct Helper<'ast>(#[serde(serialize_with = "serialize_resource")] &'ast ast::Resource<'ast>);
    Ok(serde_json::to_string(&Helper(res)).unwrap())
}

pub fn _serialize_to_pretty_json<'s>(res: &'s ast::Resource) -> Result<String, Box<Error>> {
    #[derive(Serialize)]
    struct Helper<'ast>(#[serde(serialize_with = "serialize_resource")] &'ast ast::Resource<'ast>);

    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    Helper(res).serialize(&mut ser).unwrap();
    Ok(String::from_utf8(ser.into_inner()).unwrap())
}

fn serialize_resource<'se, S>(res: &'se ast::Resource, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(
        #[serde(serialize_with = "serialize_resource_entry_vec")]
        &'ast Vec<ast::ResourceEntry<'ast>>,
    );

    let mut map = serializer.serialize_map(Some(2))?;
    map.serialize_entry("type", "Resource")?;
    map.serialize_entry("body", &Helper(&res.body))?;
    map.end()
}

fn serialize_resource_entry_vec<'se, S>(
    v: &Vec<ast::ResourceEntry<'se>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    #[serde(tag = "type")]
    enum EntryHelper<'ast> {
        Junk {
            annotations: Vec<&'ast str>,
            content: &'ast str,
        },
        #[serde(with = "MessageDef")]
        Message(&'ast ast::Message<'ast>),
        #[serde(with = "TermDef")]
        Term(&'ast ast::Term<'ast>),
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
    for e in v {
        let entry = match *e {
            ast::ResourceEntry::Entry(ref entry) => match entry {
                ast::Entry::Message(ref msg) => EntryHelper::Message(msg),
                ast::Entry::Term(ref term) => EntryHelper::Term(term),
                ast::Entry::Comment(ast::Comment::Comment { ref content }) => {
                    EntryHelper::Comment {
                        content: content.join("\n"),
                    }
                }
                ast::Entry::Comment(ast::Comment::GroupComment { ref content }) => {
                    EntryHelper::GroupComment {
                        content: content.join("\n"),
                    }
                }
                ast::Entry::Comment(ast::Comment::ResourceComment { ref content }) => {
                    EntryHelper::ResourceComment {
                        content: content.join("\n"),
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
pub struct MessageDef<'ast> {
    #[serde(serialize_with = "serialize_identifier")]
    pub id: ast::Identifier<'ast>,
    #[serde(serialize_with = "serialize_pattern_option")]
    pub value: Option<ast::Pattern<'ast>>,
    #[serde(serialize_with = "serialize_attribute_vec")]
    pub attributes: Vec<ast::Attribute<'ast>>,
    #[serde(serialize_with = "serialize_comment_option")]
    pub comment: Option<ast::Comment<'ast>>,
}

#[derive(Serialize)]
#[serde(remote = "ast::Term")]
pub struct TermDef<'ast> {
    #[serde(serialize_with = "serialize_identifier")]
    pub id: ast::Identifier<'ast>,
    #[serde(with = "ValueDef")]
    pub value: ast::Value<'ast>,
    #[serde(serialize_with = "serialize_attribute_vec")]
    pub attributes: Vec<ast::Attribute<'ast>>,
    #[serde(serialize_with = "serialize_comment_option")]
    pub comment: Option<ast::Comment<'ast>>,
}

fn serialize_pattern_option<'se, S>(
    v: &Option<ast::Pattern<'se>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(#[serde(serialize_with = "serialize_pattern")] &'ast ast::Pattern<'ast>);
    v.as_ref().map(Helper).serialize(serializer)
}

fn serialize_attribute_vec<'se, S>(
    v: &Vec<ast::Attribute<'se>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(
        #[serde(serialize_with = "serialize_attribute")] &'ast ast::Attribute<'ast>,
    );
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    for e in v {
        seq.serialize_element(&Helper(e))?;
    }
    seq.end()
}

fn serialize_comment_option<'se, S>(
    v: &Option<ast::Comment<'se>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(#[serde(with = "CommentDef")] &'ast ast::Comment<'ast>);
    v.as_ref().map(Helper).serialize(serializer)
}

#[derive(Serialize)]
#[serde(remote = "ast::Value")]
#[serde(tag = "type")]
pub enum ValueDef<'ast> {
    #[serde(serialize_with = "serialize_pattern")]
    Pattern(ast::Pattern<'ast>),
    VariantList {
        #[serde(serialize_with = "serialize_variants")]
        variants: Vec<ast::Variant<'ast>>,
    },
}

fn serialize_pattern<'se, S>(pattern: &'se ast::Pattern, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(
        #[serde(serialize_with = "serialize_pattern_elements")]
        &'ast Vec<ast::PatternElement<'ast>>,
    );

    let mut map = serializer.serialize_map(Some(2))?;
    map.serialize_entry("type", "Pattern")?;
    map.serialize_entry("elements", &Helper(&pattern.elements))?;
    map.end()
}

fn serialize_pattern_elements<'se, S>(
    v: &Vec<ast::PatternElement<'se>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(#[serde(with = "PatternElementDef")] &'ast ast::PatternElement<'ast>);
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    let mut buffer = String::new();
    for e in v {
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
#[serde(untagged)]
pub enum PatternElementDef<'ast> {
    #[serde(serialize_with = "serialize_text_element")]
    TextElement(&'ast str),
    #[serde(serialize_with = "serialize_placeable")]
    Placeable(ast::Expression<'ast>),
}

fn serialize_text_element<'se, S>(s: &'se str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(2))?;
    map.serialize_entry("type", "TextElement")?;
    map.serialize_entry("value", s)?;
    map.end()
}

fn serialize_placeable<'se, S>(exp: &ast::Expression<'se>, serializer: S) -> Result<S::Ok, S::Error>
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

fn serialize_attribute<'se, S>(
    attribute: &'se ast::Attribute,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct IdHelper<'ast>(
        #[serde(serialize_with = "serialize_identifier")] &'ast ast::Identifier<'ast>,
    );

    #[derive(Serialize)]
    struct ValueHelper<'ast>(
        #[serde(serialize_with = "serialize_pattern")] &'ast ast::Pattern<'ast>,
    );

    let mut map = serializer.serialize_map(Some(3))?;
    map.serialize_entry("type", "Attribute")?;
    map.serialize_entry("id", &IdHelper(&attribute.id))?;
    map.serialize_entry("value", &ValueHelper(&attribute.value))?;
    map.end()
}

fn serialize_identifier<'se, S>(id: &'se ast::Identifier, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(2))?;
    map.serialize_entry("type", "Identifier")?;
    map.serialize_entry("name", id.name)?;
    map.end()
}

fn serialize_variant<'se, S>(variant: &'se ast::Variant, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct KeyHelper<'ast>(#[serde(with = "VariantKeyDef")] &'ast ast::VariantKey<'ast>);

    #[derive(Serialize)]
    struct ValueHelper<'ast>(#[serde(with = "ValueDef")] &'ast ast::Value<'ast>);

    let mut map = serializer.serialize_map(Some(4))?;
    map.serialize_entry("type", "Variant")?;
    map.serialize_entry("key", &KeyHelper(&variant.key))?;
    map.serialize_entry("value", &ValueHelper(&variant.value))?;
    map.serialize_entry("default", &variant.default)?;
    map.end()
}

#[derive(Serialize, Debug)]
#[serde(remote = "ast::VariantKey")]
#[serde(tag = "type")]
pub enum VariantKeyDef<'ast> {
    Identifier { name: &'ast str },
    NumberLiteral { value: &'ast str },
}

#[derive(Serialize)]
#[serde(remote = "ast::Comment")]
#[serde(tag = "type")]
pub enum CommentDef<'ast> {
    Comment {
        #[serde(serialize_with = "serialize_comment_content")]
        content: Vec<&'ast str>,
    },
    GroupComment {
        #[serde(serialize_with = "serialize_comment_content")]
        content: Vec<&'ast str>,
    },
    ResourceComment {
        #[serde(serialize_with = "serialize_comment_content")]
        content: Vec<&'ast str>,
    },
}

fn serialize_comment_content<'se, S>(v: &Vec<&'se str>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&v.join("\n"))
}

#[derive(Serialize)]
#[serde(remote = "ast::InlineExpression")]
#[serde(tag = "type")]
pub enum InlineExpressionDef<'ast> {
    #[serde(serialize_with = "serialize_string_literal")]
    StringLiteral {
        raw: &'ast str,
    },
    NumberLiteral {
        value: &'ast str,
    },
    VariableReference {
        #[serde(serialize_with = "serialize_identifier")]
        id: ast::Identifier<'ast>,
    },
    CallExpression {
        #[serde(with = "InlineExpressionDef")]
        callee: ast::InlineExpression<'ast>,
        #[serde(serialize_with = "serialize_inline_expressions")]
        positional: Vec<ast::InlineExpression<'ast>>,
        #[serde(serialize_with = "serialize_named_arguments")]
        named: Vec<ast::NamedArgument<'ast>>,
    },
    AttributeExpression {
        #[serde(with = "InlineExpressionDef")]
        #[serde(rename = "ref")]
        reference: ast::InlineExpression<'ast>,
        #[serde(serialize_with = "serialize_identifier")]
        name: ast::Identifier<'ast>,
    },
    VariantExpression {
        #[serde(with = "InlineExpressionDef")]
        #[serde(rename = "ref")]
        reference: ast::InlineExpression<'ast>,
        #[serde(with = "VariantKeyDef")]
        key: ast::VariantKey<'ast>,
    },
    MessageReference {
        #[serde(serialize_with = "serialize_identifier")]
        id: ast::Identifier<'ast>,
    },
    TermReference {
        #[serde(serialize_with = "serialize_identifier")]
        id: ast::Identifier<'ast>,
    },
    FunctionReference {
        #[serde(serialize_with = "serialize_identifier")]
        id: ast::Identifier<'ast>,
    },
    Placeable {
        #[serde(with = "ExpressionDef")]
        expression: ast::Expression<'ast>,
    },
}

fn serialize_string_literal<'se, S>(raw: &'se str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(3))?;
    map.serialize_entry("type", "StringLiteral")?;
    map.serialize_entry("raw", raw)?;
    map.serialize_entry("value", &helper::unescape_unicode(&raw))?;
    map.end()
}

fn serialize_named_argument<'se, S>(
    arg: &'se ast::NamedArgument,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct IdentifierHelper<'ast>(
        #[serde(serialize_with = "serialize_identifier")] &'ast ast::Identifier<'ast>,
    );

    #[derive(Serialize)]
    struct InlineExpressionHelper<'ast>(
        #[serde(with = "InlineExpressionDef")] &'ast ast::InlineExpression<'ast>,
    );

    let mut map = serializer.serialize_map(Some(3))?;
    map.serialize_entry("type", "NamedArgument")?;
    map.serialize_entry("name", &IdentifierHelper(&arg.name))?;
    map.serialize_entry("value", &InlineExpressionHelper(&arg.value))?;
    map.end()
}

#[derive(Serialize)]
#[serde(remote = "ast::Expression")]
#[serde(tag = "type")]
pub enum ExpressionDef<'ast> {
    #[serde(with = "InlineExpressionDef")]
    InlineExpression(ast::InlineExpression<'ast>),
    SelectExpression {
        #[serde(with = "InlineExpressionDef")]
        selector: ast::InlineExpression<'ast>,
        #[serde(serialize_with = "serialize_variants")]
        variants: Vec<ast::Variant<'ast>>,
    },
}

fn serialize_variants<'se, S>(v: &Vec<ast::Variant<'se>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(#[serde(serialize_with = "serialize_variant")] &'ast ast::Variant<'ast>);
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    for e in v {
        seq.serialize_element(&Helper(e))?;
    }
    seq.end()
}

fn serialize_inline_expressions<'se, S>(
    v: &Vec<ast::InlineExpression<'se>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(#[serde(with = "InlineExpressionDef")] &'ast ast::InlineExpression<'ast>);
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    for e in v {
        seq.serialize_element(&Helper(e))?;
    }
    seq.end()
}

fn serialize_named_arguments<'se, S>(
    v: &Vec<ast::NamedArgument<'se>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'ast>(
        #[serde(serialize_with = "serialize_named_argument")] &'ast ast::NamedArgument<'ast>,
    );
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    for e in v {
        seq.serialize_element(&Helper(e))?;
    }
    seq.end()
}
