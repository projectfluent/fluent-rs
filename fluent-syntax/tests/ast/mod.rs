use fluent_syntax::ast;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use serde_derive::Serialize;
use std::error::Error;

pub fn serialize<'s>(res: &'s ast::Resource) -> Result<String, Box<Error>> {
    #[derive(Serialize)]
    struct Helper<'ast>(#[serde(with = "ResourceDef")] &'ast ast::Resource<'ast>);

    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    Helper(res).serialize(&mut ser).unwrap();
    Ok(String::from_utf8(ser.into_inner()).unwrap())
}

#[derive(Serialize, Debug)]
#[serde(remote = "ast::Resource")]
struct ResourceDef<'ast> {
    #[serde(serialize_with = "serialize_resource_entry_vec")]
    body: Vec<ast::ResourceEntry<'ast>>,
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
    #[serde(with = "IdentifierDef")]
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
    #[serde(with = "IdentifierDef")]
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
    struct Helper<'ast>(#[serde(with = "PatternDef")] &'ast ast::Pattern<'ast>);
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
    struct Helper<'ast>(#[serde(with = "AttributeDef")] &'ast ast::Attribute<'ast>);
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
#[serde(untagged)]
pub enum ValueDef<'ast> {
    #[serde(with = "PatternDef")]
    Pattern(ast::Pattern<'ast>),
    VariantList {
        #[serde(serialize_with = "serialize_variants")]
        variants: Vec<ast::Variant<'ast>>,
    },
}

#[derive(Serialize)]
#[serde(remote = "ast::Pattern")]
pub struct PatternDef<'ast> {
    #[serde(serialize_with = "serialize_pattern_elements")]
    pub elements: Vec<ast::PatternElement<'ast>>,
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
    for e in v {
        seq.serialize_element(&Helper(e))?;
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

#[derive(Serialize)]
#[serde(remote = "ast::Attribute")]
pub struct AttributeDef<'ast> {
    #[serde(with = "IdentifierDef")]
    pub id: ast::Identifier<'ast>,
    #[serde(with = "PatternDef")]
    pub value: ast::Pattern<'ast>,
}

#[derive(Serialize, Debug)]
#[serde(remote = "ast::Identifier")]
struct IdentifierDef<'ast> {
    name: &'ast str,
}

#[derive(Serialize, Debug)]
#[serde(remote = "ast::Function")]
struct FunctionDef<'ast> {
    name: &'ast str,
}

#[derive(Serialize, Debug)]
#[serde(remote = "ast::Variant")]
struct VariantDef<'ast> {
    #[serde(with = "VariantKeyDef")]
    pub key: ast::VariantKey<'ast>,
    #[serde(with = "ValueDef")]
    pub value: ast::Value<'ast>,
    pub default: bool,
}

#[derive(Serialize, Debug)]
#[serde(remote = "ast::VariantKey")]
#[serde(untagged)]
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
    StringLiteral {
        value: &'ast str,
    },
    NumberLiteral {
        value: &'ast str,
    },
    VariableReference {
        #[serde(with = "IdentifierDef")]
        id: ast::Identifier<'ast>,
    },
    CallExpression {
        #[serde(with = "FunctionDef")]
        callee: ast::Function<'ast>,
        #[serde(serialize_with = "serialize_inline_expressions")]
        positional: Vec<ast::InlineExpression<'ast>>,
        #[serde(serialize_with = "serialize_named_arguments")]
        named: Vec<ast::NamedArgument<'ast>>,
    },
    AttributeExpression {
        #[serde(with = "InlineExpressionDef")]
        #[serde(rename = "ref")]
        reference: ast::InlineExpression<'ast>,
        #[serde(with = "IdentifierDef")]
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
        #[serde(with = "IdentifierDef")]
        id: ast::Identifier<'ast>,
    },
    TermReference {
        #[serde(with = "IdentifierDef")]
        id: ast::Identifier<'ast>,
    },
    Placeable {
        #[serde(with = "ExpressionDef")]
        expression: ast::Expression<'ast>,
    },
}

#[derive(Serialize)]
#[serde(remote = "ast::NamedArgument")]
pub struct NamedArgumentDef<'ast> {
    #[serde(with = "IdentifierDef")]
    pub name: ast::Identifier<'ast>,
    #[serde(with = "InlineExpressionDef")]
    pub value: ast::InlineExpression<'ast>,
}

#[derive(Serialize)]
#[serde(remote = "ast::Expression")]
#[serde(untagged)]
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
    struct Helper<'ast>(#[serde(with = "VariantDef")] &'ast ast::Variant<'ast>);
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
    struct Helper<'ast>(#[serde(with = "NamedArgumentDef")] &'ast ast::NamedArgument<'ast>);
    let mut seq = serializer.serialize_seq(Some(v.len()))?;
    for e in v {
        seq.serialize_element(&Helper(e))?;
    }
    seq.end()
}
