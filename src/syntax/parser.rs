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

    ps.skip_blank_lines();

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

        ps.skip_blank_lines();
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

    if ps.is_entry_id_start() {
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

fn get_comment<I>(ps: &mut ParserStream<I>) -> Result<ast::Comment>
where
    I: Iterator<Item = char>,
{
    let mut level = -1;
    let mut content = String::new();

    loop {
        let mut i = -1;
        while ps.current_is('#') && ((level == -1 && i < 2) || (level != -1 && i < level)) {
            ps.next();
            i += 1;
        }

        if level == -1 {
            level = i;
        }

        if !ps.current_is('\n') {
            ps.expect_char(' ')?;
            while let Some(ch) = ps.take_char(|x| x != '\n') {
                content.push(ch);
            }
        }

        if ps.is_peek_next_line_comment(level) {
            content.push('\n');
            ps.next();
        } else {
            break;
        }
    }

    match level {
        0 => Ok(ast::Comment::Comment { content }),
        1 => Ok(ast::Comment::GroupComment { content }),
        2 => Ok(ast::Comment::ResourceComment { content }),
        _ => panic!("Unknown comment level!"),
    }
}

fn get_message<I>(ps: &mut ParserStream<I>, comment: Option<ast::Comment>) -> Result<ast::Entry>
where
    I: Iterator<Item = char>,
{
    let id = get_entry_identifier(ps)?;

    ps.skip_inline_ws();

    ps.expect_char('=')?;

    let pattern = if ps.is_peek_pattern_start() {
        ps.skip_indent();
        get_pattern(ps)?
    } else {
        None
    };

    let attributes = if ps.is_peek_next_line_attribute_start() {
        Some(get_attributes(ps)?)
    } else {
        None
    };

    if id.name.starts_with('-') {
        match pattern {
            Some(pattern) => {
                return Ok(ast::Entry::Term(ast::Term {
                    id,
                    value: pattern,
                    attributes,
                    comment,
                }));
            }
            None => {
                return error!(ErrorKind::ExpectedTermField { entry_id: id.name });
            }
        }
    }

    if pattern.is_none() && attributes.is_none() {
        return error!(ErrorKind::ExpectedMessageField { entry_id: id.name });
    }

    Ok(ast::Entry::Message(ast::Message {
        id,
        value: pattern,
        attributes,
        comment,
    }))
}

fn get_attribute<I>(ps: &mut ParserStream<I>) -> Result<ast::Attribute>
where
    I: Iterator<Item = char>,
{
    ps.expect_char('.')?;

    let key = get_identifier(ps, false)?;

    ps.skip_inline_ws();
    ps.expect_char('=')?;

    if ps.is_peek_pattern_start() {
        ps.skip_indent();
        let value = get_pattern(ps)?;
        if let Some(value) = value {
            return Ok(ast::Attribute { id: key, value });
        }
    }
    error!(ErrorKind::MissingValue)
}

fn get_attributes<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Attribute>>
where
    I: Iterator<Item = char>,
{
    let mut attributes = vec![];
    loop {
        ps.expect_indent()?;
        let attr = get_attribute(ps)?;
        attributes.push(attr);

        if !ps.is_peek_next_line_attribute_start() {
            break;
        }
    }
    Ok(attributes)
}

fn get_entry_identifier<I>(ps: &mut ParserStream<I>) -> Result<ast::Identifier>
where
    I: Iterator<Item = char>,
{
    get_identifier(ps, true)
}

fn get_identifier<I>(ps: &mut ParserStream<I>, allow_term: bool) -> Result<ast::Identifier>
where
    I: Iterator<Item = char>,
{
    let mut name = String::new();

    name.push(ps.take_id_start(allow_term)?);

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
        return error!(ErrorKind::MissingVariantKey);
    }
}

fn get_variant<I>(ps: &mut ParserStream<I>, has_default: bool) -> Result<ast::Variant>
where
    I: Iterator<Item = char>,
{
    let mut default_index = false;

    if ps.current_is('*') {
        if has_default {
            return error!(ErrorKind::MultipleDefaultVariants);
        }
        ps.next();
        default_index = true;
    }

    ps.expect_char('[')?;

    let key = get_variant_key(ps)?;

    ps.expect_char(']')?;

    if ps.is_peek_pattern_start() {
        ps.skip_indent();
        if let Some(value) = get_pattern(ps)? {
            return Ok(ast::Variant {
                key,
                value,
                default: default_index,
            });
        }
    }
    return error!(ErrorKind::MissingValue);
}

fn get_variants<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Variant>>
where
    I: Iterator<Item = char>,
{
    let mut variants = vec![];
    let mut has_default = false;

    loop {
        ps.expect_indent()?;
        let variant = get_variant(ps, has_default)?;

        if variant.default {
            has_default = true;
        }

        variants.push(variant);

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

    while let Some(ch) = ps.take_variant_name_char() {
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

    while let Some(ch) = ps.take_digit() {
        num.push(ch);
    }

    if num.is_empty() {
        return error!(ErrorKind::ExpectedCharRange {
            range: "0...9".to_owned(),
        });
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
    let mut elements = vec![];

    ps.skip_inline_ws();

    while let Some(ch) = ps.current() {
        if ch == '\n' && !ps.is_peek_next_line_pattern_start() {
            break;
        }

        match ch {
            '{' => {
                elements.push(get_placeable(ps)?);
            }
            _ => {
                elements.push(get_text_element(ps)?);
            }
        }
    }

    Ok(Some(ast::Pattern { elements }))
}

fn get_text_element<I>(ps: &mut ParserStream<I>) -> Result<ast::PatternElement>
where
    I: Iterator<Item = char>,
{
    let mut buf = String::new();

    while let Some(ch) = ps.current() {
        match ch {
            '{' => return Ok(ast::PatternElement::TextElement(buf)),
            '\n' => {
                if !ps.is_peek_next_line_pattern_start() {
                    return Ok(ast::PatternElement::TextElement(buf));
                }
                ps.next();
                ps.skip_inline_ws();

                // Add the new line to the buffer
                buf.push(ch);
                continue;
            }
            '\\' => {
                if let Some(ch2) = ps.next() {
                    if ch2 == '{' || ch2 == '"' {
                        buf.push(ch2);
                    } else {
                        buf.push(ch);
                        buf.push(ch2);
                    }
                }
            }
            _ => buf.push(ch),
        }
        ps.next();
    }

    Ok(ast::PatternElement::TextElement(buf))
}

fn get_placeable<I>(ps: &mut ParserStream<I>) -> Result<ast::PatternElement>
where
    I: Iterator<Item = char>,
{
    ps.expect_char('{')?;
    let expression = get_expression(ps)?;
    ps.expect_char('}')?;
    Ok(ast::PatternElement::Placeable(expression))
}

fn get_expression<I>(ps: &mut ParserStream<I>) -> Result<ast::Expression>
where
    I: Iterator<Item = char>,
{
    if ps.is_peek_next_line_variant_start() {
        let variants = get_variants(ps)?;

        ps.expect_indent()?;

        return Ok(ast::Expression::SelectExpression {
            expression: None,
            variants,
        });
    }

    ps.skip_inline_ws();

    let selector = get_selector_expression(ps)?;

    ps.skip_inline_ws();

    if ps.current_is('-') {
        ps.peek();

        if !ps.current_peek_is('>') {
            ps.reset_peek(None);
            return Ok(selector);
        }

        match selector {
            ast::Expression::MessageReference { .. } => {
                return error!(ErrorKind::MessageReferenceAsSelector)
            }
            ast::Expression::AttributeExpression { ref id, .. } => {
                if !id.name.starts_with('-') {
                    return error!(ErrorKind::MessageAttributeAsSelector);
                }
            }
            ast::Expression::VariantExpression { .. } => {
                return error!(ErrorKind::VariantAsSelector)
            }
            _ => {}
        };

        ps.next();
        ps.next();

        ps.skip_inline_ws();

        let variants = get_variants(ps)?;

        if variants.is_empty() {
            return error!(ErrorKind::MissingVariants);
        }

        ps.expect_indent()?;

        return Ok(ast::Expression::SelectExpression {
            expression: Some(Box::new(selector)),
            variants,
        });
    } else if let ast::Expression::AttributeExpression { ref id, .. } = selector {
        if id.name.starts_with('-') {
            return error!(ErrorKind::TermAttributeAsSelector);
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
                let attr = get_identifier(ps, false)?;
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

                // XXX Make sure that id.name is [A-Z][A-Z_?-]*
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

fn get_call_arg<I>(ps: &mut ParserStream<I>) -> Result<ast::Argument>
where
    I: Iterator<Item = char>,
{
    let exp = get_selector_expression(ps)?;

    if !ps.current_is(':') {
        return Ok(ast::Argument::Expression(exp));
    }

    match exp {
        ast::Expression::MessageReference { id } => {
            ps.next();
            ps.skip_inline_ws();

            let val = get_arg_val(ps)?;
            Ok(ast::Argument::NamedArgument { name: id, val })
        }
        _ => {
            error!(ErrorKind::ForbiddenKey)
        }
    }
}

fn get_call_args<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Argument>>
where
    I: Iterator<Item = char>,
{
    let mut args = vec![];

    ps.skip_inline_ws();

    loop {
        if ps.current_is(')') {
            break;
        }

        let arg = get_call_arg(ps)?;
        args.push(arg);

        ps.skip_inline_ws();

        if ps.current_is(',') {
            ps.next();
            ps.skip_inline_ws();
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
    if ps.is_number_start() {
        return Ok(ast::ArgValue::Number(get_number(ps)?));
    } else if ps.current_is('"') {
        return Ok(ast::ArgValue::String(get_string(ps)?));
    }
    error!(ErrorKind::MissingValue)
}

fn get_string<I>(ps: &mut ParserStream<I>) -> Result<String>
where
    I: Iterator<Item = char>,
{
    let mut val = String::new();

    ps.expect_char('"')?;

    while let Some(ch) = ps.take_char(|x| x != '"' && x != '\n') {
        val.push(ch);
    }

    if ps.current_is('\n') {
        return error!(ErrorKind::UnterminatedStringExpression);
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
                ps.reset_peek(None);
                ast::Expression::NumberExpression {
                    value: get_number(ps)?,
                }
            } else {
                ps.reset_peek(None);
                ast::Expression::MessageReference {
                    id: get_entry_identifier(ps)?,
                }
            },
            '"' => ast::Expression::StringExpression {
                value: get_string(ps)?,
            },
            '$' => {
                ps.next();
                ast::Expression::ExternalArgument {
                    id: get_identifier(ps, false)?,
                }
            }
            _ => ast::Expression::MessageReference {
                id: get_entry_identifier(ps)?,
            },
        };
        Ok(exp)
    } else {
        return error!(ErrorKind::MissingLiteral);
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
