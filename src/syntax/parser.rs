pub use super::errors::ParserError;
pub use super::errors::ErrorKind;
pub use super::errors::get_error_slice;
pub use super::errors::get_error_info;

use super::stream::ParserStream;
use super::ftlstream::FTLParserStream;

use std::result;

use super::ast;

pub type Result<T> = result::Result<T, ParserError>;

pub fn parse(source: &str) -> result::Result<ast::Resource, (ast::Resource, Vec<ParserError>)> {
    let mut errors = vec![];

    let mut ps = ParserStream::new(source.chars());

    ps.skip_ws_lines();

    let mut entries = vec![];

    while ps.current().is_some() {
        let entry_start_pos = ps.get_index();

        match get_entry(&mut ps) {
            Ok(entry) => {
                entries.push(entry);
            }
            Err(mut e) => {
                let error_pos = ps.get_index();
                entries.push(get_junk_entry(&mut ps, source, entry_start_pos));

                e.info = get_error_info(source, error_pos, entry_start_pos, ps.get_index());
                errors.push(e);
            }
        }

        ps.skip_ws_lines();
    }

    if errors.is_empty() {
        Ok(ast::Resource { body: entries })
    } else {
        Err((ast::Resource { body: entries }, errors))
    }
}

fn get_entry<I>(ps: &mut ParserStream<I>) -> Result<ast::Entry>
where
    I: Iterator<Item = char>,
{
    let comment = if ps.current_is('#') {
        Some(get_comment(ps)?)
    } else {
        None
    };

    if ps.is_message_id_start() {
        match comment {
            None | Some(ast::Comment::Comment { .. }) => {
                return Ok(get_message(ps, comment)?);
            }
            _ => {}
        };
    }

    match comment {
        Some(comment) => Ok(ast::Entry::Comment(comment)),
        None => error!(ErrorKind::ExpectedEntry),
    }
}

#[derive(PartialEq, Copy, Clone)]
enum CommentLevel {
    Comment = 0,
    GroupComment = 1,
    ResourceComment = 2,
}

fn get_comment_start<I>(ps: &mut ParserStream<I>, level: &CommentLevel) -> Result<()>
where
    I: Iterator<Item = char>,
{
    let depth = *level as u8;
    for _ in 0..(depth + 1) {
        ps.expect_char('#')?;
    }

    if !ps.current_is('\n') {
        ps.expect_char(' ')?;
    }
    Ok(())
}

fn get_comment_level<I>(ps: &mut ParserStream<I>) -> Result<CommentLevel>
where
    I: Iterator<Item = char>,
{
    let mut level = CommentLevel::Comment;
    ps.peek();
    if ps.current_peek_is('#') {
        ps.peek();
        if ps.current_peek_is('#') {
            level = CommentLevel::ResourceComment;
        } else {
            level = CommentLevel::GroupComment;
        }
    }
    ps.reset_peek();
    Ok(level)
}

fn get_comment<I>(ps: &mut ParserStream<I>) -> Result<ast::Comment>
where
    I: Iterator<Item = char>,
{
    let level = get_comment_level(ps)?;
    get_comment_start(ps, &level)?;

    let mut content = String::new();

    loop {
        while let Some(ch) = ps.take_char(|x| x != '\n') {
            content.push(ch);
        }

        ps.next();

        if !ps.current_is('#') || level != get_comment_level(ps)? {
            break;
        } else {
            get_comment_start(ps, &level)?;
        }
    }

    match level {
        CommentLevel::Comment => Ok(ast::Comment::Comment { content }),
        CommentLevel::GroupComment => Ok(ast::Comment::GroupComment { content }),
        CommentLevel::ResourceComment => Ok(ast::Comment::ResourceComment { content }),
    }
}

fn get_message<I>(ps: &mut ParserStream<I>, comment: Option<ast::Comment>) -> Result<ast::Entry>
where
    I: Iterator<Item = char>,
{
    let id = get_private_identifier(ps)?;

    ps.skip_line_ws();

    let pattern = if ps.current_is('=') {
        ps.next();

        ps.skip_line_ws();

        get_pattern(ps)?
    } else {
        None
    };

    let attributes = if ps.is_peek_next_line_attribute_start() {
        Some(get_attributes(ps)?)
    } else {
        None
    };

    if pattern.is_none() && attributes.is_none() {
        return error!(ErrorKind::MissingField {
            entry_id: id.name,
            fields: vec!["Value", "Attribute"],
        });
    }

    Ok(ast::Entry::Message(ast::Message {
        id,
        value: pattern,
        attributes,
        comment,
    }))
}

fn get_attributes<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Attribute>>
where
    I: Iterator<Item = char>,
{
    let mut attributes = vec![];
    loop {
        ps.expect_char('\n')?;
        ps.skip_line_ws();

        ps.expect_char('.')?;

        let key = get_public_identifier(ps)?;

        ps.skip_line_ws();

        ps.expect_char('=')?;

        ps.skip_line_ws();

        if let Some(pattern) = get_pattern(ps)? {
            attributes.push(ast::Attribute {
                id: key,
                value: pattern,
            });
        } else {
            return error!(ErrorKind::ExpectedField {
                field: String::from("Pattern"),
            });
        }

        if !ps.is_peek_next_line_attribute_start() {
            break;
        }
    }
    Ok(attributes)
}

fn get_private_identifier<I>(ps: &mut ParserStream<I>) -> Result<ast::Identifier>
where
    I: Iterator<Item = char>,
{
    get_identifier(ps, true)
}

fn get_public_identifier<I>(ps: &mut ParserStream<I>) -> Result<ast::Identifier>
where
    I: Iterator<Item = char>,
{
    get_identifier(ps, false)
}

fn get_identifier<I>(ps: &mut ParserStream<I>, allow_private: bool) -> Result<ast::Identifier>
where
    I: Iterator<Item = char>,
{
    let mut name = String::new();

    name.push(ps.take_id_start(allow_private)?);

    while let Some(ch) = ps.take_id_char() {
        name.push(ch);
    }

    Ok(ast::Identifier { name })
}

fn get_variant_key<I>(ps: &mut ParserStream<I>) -> Result<ast::VarKey>
where
    I: Iterator<Item = char>,
{
    if let Some(ch) = ps.current() {
        match ch {
            '0'...'9' | '-' => {
                return Ok(ast::VarKey::Number(get_number(ps)?));
            }
            _ => {
                return Ok(ast::VarKey::VariantName(get_variant_name(ps)?));
            }
        }
    } else {
        return error!(ErrorKind::ExpectedField {
            field: "VariantName | Number".to_owned(),
        });
    }
}

fn get_variants<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Variant>>
where
    I: Iterator<Item = char>,
{
    let mut variants = vec![];
    let mut has_default = false;

    loop {
        let mut default_index = false;

        ps.expect_char('\n')?;
        ps.skip_line_ws();

        if ps.current_is('*') {
            if has_default {
                return error!(ErrorKind::Generic);
            }
            ps.next();
            default_index = true;
            has_default = true;
        }

        ps.expect_char('[')?;

        let key = get_variant_key(ps)?;

        ps.expect_char(']')?;

        ps.skip_line_ws();

        if let Some(pattern) = get_pattern(ps)? {
            variants.push(ast::Variant {
                key,
                value: pattern,
                default: default_index,
            });
        } else {
            return error!(ErrorKind::ExpectedField {
                field: String::from("Pattern"),
            });
        }

        if !ps.is_peek_next_line_variant_start() {
            break;
        }
    }
    if !has_default {
        return error!(ErrorKind::MissingDefaultVariant);
    }
    Ok(variants)
}

fn get_variant_name<I>(ps: &mut ParserStream<I>) -> Result<ast::VariantName>
where
    I: Iterator<Item = char>,
{
    let mut name = String::new();

    name.push(ps.take_id_start(false)?);

    while let Some(ch) = ps.take_symb_char() {
        name.push(ch);
    }

    while name.ends_with(' ') {
        name.pop();
    }

    Ok(ast::VariantName { name })
}

fn get_digits<I>(ps: &mut ParserStream<I>) -> Result<String>
where
    I: Iterator<Item = char>,
{
    let mut num = String::new();

    if let Some(ch) = ps.current() {
        match ch {
            '0'...'9' => {
                num.push(ch);
                ps.next();
            }
            _ => {
                return error!(ErrorKind::ExpectedCharRange {
                    range: "0...9".to_owned(),
                })
            }
        }
    } else {
        return error!(ErrorKind::ExpectedCharRange {
            range: "0...9".to_owned(),
        });
    }

    while let Some(ch) = ps.current() {
        match ch {
            '0'...'9' => {
                num.push(ch);
                ps.next();
            }
            _ => {
                break;
            }
        }
    }

    Ok(num)
}

fn get_number<I>(ps: &mut ParserStream<I>) -> Result<ast::Number>
where
    I: Iterator<Item = char>,
{
    let mut num = String::new();

    if ps.current_is('-') {
        num.push('-');
        ps.next();
    }

    num.push_str(&get_digits(ps)?);

    if ps.current_is('.') {
        num.push('.');
        ps.next();
        num.push_str(&get_digits(ps)?);
    }
    Ok(ast::Number { value: num })
}

fn get_pattern<I>(ps: &mut ParserStream<I>) -> Result<Option<ast::Pattern>>
where
    I: Iterator<Item = char>,
{
    let mut buffer = String::new();
    let mut elements = vec![];
    let mut first_line = true;

    while let Some(ch) = ps.current() {
        match ch {
            '\n' => {
                if first_line && !buffer.is_empty() {
                    break;
                }

                if !ps.is_peek_next_line_pattern() {
                    break;
                }

                ps.next();
                ps.skip_line_ws();

                if !first_line {
                    buffer.push(ch);
                }

                first_line = false;
                continue;
            }
            '\\' => if let Some(ch2) = ps.peek() {
                match ch2 {
                    '{' => {
                        buffer.push(ch2);
                        ps.next();
                    }
                    _ => {
                        buffer.push(ch);
                        buffer.push(ch2);
                        ps.next();
                    }
                }
            } else {
                ps.reset_peek();
                buffer.push(ch);
                break;
            },
            '{' => {
                ps.next();

                ps.skip_line_ws();

                if !buffer.is_empty() {
                    elements.push(ast::PatternElement::TextElement(buffer));
                }

                buffer = String::new();

                elements.push(ast::PatternElement::Placeable(ast::Placeable {
                    expression: get_expression(ps)?,
                }));

                ps.expect_char('}')?;

                continue;
            }
            _ => {
                buffer.push(ch);
            }
        }
        ps.next();
    }

    if !buffer.is_empty() {
        elements.push(ast::PatternElement::TextElement(buffer));
    }

    Ok(Some(ast::Pattern { elements }))
}

fn get_expression<I>(ps: &mut ParserStream<I>) -> Result<ast::Expression>
where
    I: Iterator<Item = char>,
{
    if ps.is_peek_next_line_variant_start() {
        let variants = get_variants(ps)?;

        ps.expect_char('\n')?;
        ps.expect_char(' ')?;
        ps.skip_line_ws();

        return Ok(ast::Expression::SelectExpression {
            expression: None,
            variants,
        });
    }

    let selector = get_selector_expression(ps)?;

    ps.skip_line_ws();

    if ps.current_is('-') {
        if let Some('>') = ps.peek() {
            if let ast::Expression::AttributeExpression { ref id, .. } = selector {
                if !id.name.starts_with('-') {
                    return error!(ErrorKind::ForbiddenPublicAttributeExpression);
                }
            } else if let ast::Expression::VariantExpression { .. } = selector {
                return error!(ErrorKind::ForbiddenVariantExpression);
            }
            ps.next();
            ps.next();

            ps.skip_line_ws();

            let variants = get_variants(ps)?;

            if variants.is_empty() {
                return error!(ErrorKind::MissingVariants);
            }

            ps.expect_char('\n')?;
            ps.expect_char(' ')?;
            ps.skip_line_ws();

            return Ok(ast::Expression::SelectExpression {
                expression: Some(Box::new(selector)),
                variants,
            });
        } else {
            ps.reset_peek();
        }
    } else {
        if let ast::Expression::AttributeExpression { ref id, .. } = selector {
            if id.name.starts_with('-') {
                return error!(ErrorKind::ForbiddenPrivateAttributeExpression);
            }
        }
    }

    Ok(selector)
}

fn get_selector_expression<I>(ps: &mut ParserStream<I>) -> Result<ast::Expression>
where
    I: Iterator<Item = char>,
{
    let literal = get_literal(ps)?;

    match literal {
        ast::Expression::MessageReference { id } => match ps.ch {
            Some('.') => {
                ps.next();
                let attr = get_public_identifier(ps)?;
                Ok(ast::Expression::AttributeExpression { id, name: attr })
            }
            Some('[') => {
                ps.next();
                let key = get_variant_key(ps)?;
                ps.expect_char(']')?;

                Ok(ast::Expression::VariantExpression { id, key: key })
            }
            Some('(') => {
                if id.name.starts_with('-') || id.name.chars().any(|c| c.is_lowercase()) {
                    return error!(ErrorKind::ForbiddenCallee);
                }
                ps.next();
                let args = get_call_args(ps)?;
                ps.expect_char(')')?;
                Ok(ast::Expression::CallExpression {
                    callee: ast::Function { name: id.name },
                    args: args,
                })
            }
            _ => Ok(ast::Expression::MessageReference { id: id }),
        },
        _ => Ok(literal),
    }
}

fn get_call_args<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Argument>>
where
    I: Iterator<Item = char>,
{
    let mut args = vec![];

    ps.skip_line_ws();

    loop {
        if ps.current_is(')') {
            break;
        }
        let exp = get_selector_expression(ps)?;

        ps.skip_line_ws();

        //XXX: Here!!!
        if ps.current_is(':') {
            match exp {
                ast::Expression::MessageReference { id } => {
                    ps.next();
                    ps.skip_line_ws();

                    let val = get_arg_val(ps)?;
                    args.push(ast::Argument::NamedArgument { name: id, val: val });
                }
                _ => {
                    return error!(ErrorKind::ForbiddenKey);
                }
            }
        } else {
            args.push(ast::Argument::Expression(exp));
        }

        ps.skip_line_ws();

        if ps.current_is(',') {
            ps.next();
            ps.skip_line_ws();
            continue;
        } else {
            break;
        }
    }

    Ok(args)
}

fn get_arg_val<I>(ps: &mut ParserStream<I>) -> Result<ast::ArgValue>
where
    I: Iterator<Item = char>,
{
    if let Some(ch) = ps.current() {
        match ch {
            '0'...'9' | '-' => Ok(ast::ArgValue::Number(get_number(ps)?)),
            '"' => Ok(ast::ArgValue::String(get_string(ps)?)),
            _ => error!(ErrorKind::ExpectedField {
                field: String::from("Argument value"),
            }),
        }
    } else {
        error!(ErrorKind::ExpectedField {
            field: String::from("Literal"),
        })
    }
}

fn get_string<I>(ps: &mut ParserStream<I>) -> Result<String>
where
    I: Iterator<Item = char>,
{
    let mut val = String::new();

    ps.expect_char('"')?;

    while let Some(ch) = ps.take_char(|x| x != '"') {
        val.push(ch);
    }

    ps.next();

    Ok(val)
}

fn get_literal<I>(ps: &mut ParserStream<I>) -> Result<ast::Expression>
where
    I: Iterator<Item = char>,
{
    if let Some(ch) = ps.current() {
        let exp = match ch {
            '0'...'9' => ast::Expression::NumberExpression {
                value: get_number(ps)?,
            },
            '-' => if let Some('0'...'9') = ps.peek() {
                ps.reset_peek();
                ast::Expression::NumberExpression {
                    value: get_number(ps)?,
                }
            } else {
                ps.reset_peek();
                ast::Expression::MessageReference {
                    id: get_private_identifier(ps)?,
                }
            },
            '"' => ast::Expression::StringExpression {
                value: get_string(ps)?,
            },
            '$' => {
                ps.next();
                ast::Expression::ExternalArgument {
                    id: get_public_identifier(ps)?,
                }
            }
            _ => ast::Expression::MessageReference {
                id: get_private_identifier(ps)?,
            },
        };
        Ok(exp)
    } else {
        return error!(ErrorKind::ExpectedField {
            field: String::from("Literal"),
        });
    }
}

fn get_junk_entry<I>(ps: &mut ParserStream<I>, source: &str, entry_start: usize) -> ast::Entry
where
    I: Iterator<Item = char>,
{
    ps.skip_to_next_entry_start();

    let slice = get_error_slice(source, entry_start, ps.get_index());

    ast::Entry::Junk {
        content: String::from(slice),
    }
}
