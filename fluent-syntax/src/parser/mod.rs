#[macro_use]
pub mod errors;
mod ftlstream;

use std::result;
use std::str;

use self::errors::ErrorKind;
pub use self::errors::ParserError;
use self::ftlstream::ParserStream;
use super::ast;

pub type Result<T> = result::Result<T, ParserError>;

pub fn parse(source: &str) -> result::Result<ast::Resource, (ast::Resource, Vec<ParserError>)> {
    let mut errors = vec![];

    let mut ps = ParserStream::new(source);

    let mut body = vec![];

    ps.skip_blank_block();

    let mut last_entry_end = ps.ptr;
    let mut last_comment = None;

    while ps.ptr < ps.length {
        let entry_start = ps.ptr;
        match get_entry(&mut ps) {
            Ok(entry) => {
                if last_entry_end != entry_start {
                    let mut te = 0;
                    while ps.is_byte_at(b'\n', entry_start - te - 1) {
                        te += 1;
                    }
                    let te = if te > 1 { te - 1 } else { 0 };
                    let slice = ps.get_slice(last_entry_end, entry_start - te);
                    body.push(ast::ResourceEntry::Junk(slice));
                }
                if let Some(content) = last_comment {
                    match entry {
                        ast::Entry::Message(mut msg) => {
                            msg.comment = Some(ast::Comment::Comment { content });
                            body.push(ast::ResourceEntry::Entry(ast::Entry::Message(msg)));
                            last_comment = None;
                        }
                        ast::Entry::Term(mut term) => {
                            term.comment = Some(ast::Comment::Comment { content });
                            body.push(ast::ResourceEntry::Entry(ast::Entry::Term(term)));
                            last_comment = None;
                        }
                        ast::Entry::Comment(new_comment) => {
                            body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(
                                ast::Comment::Comment { content },
                            )));
                            if let ast::Comment::Comment { content } = new_comment {
                                last_comment = Some(content);
                            } else {
                                body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(
                                    new_comment,
                                )));
                                last_comment = None;
                            }
                        }
                    }
                } else {
                    match entry {
                        ast::Entry::Comment(ast::Comment::Comment { content }) => {
                            last_comment = Some(content);
                        }
                        _ => {
                            body.push(ast::ResourceEntry::Entry(entry));
                        }
                    }
                }
                ps.skip_eol();
                if ps.skip_blank_block() > 0 {
                    if let Some(content) = last_comment {
                        body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(
                            ast::Comment::Comment { content },
                        )));
                        last_comment = None;
                    }
                }
                last_entry_end = ps.ptr;
            }
            Err(mut err) => {
                if let Some(content) = last_comment {
                    body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(
                        ast::Comment::Comment { content },
                    )));
                    last_comment = None;
                }
                ps.skip_to_next_entry_start();
                let mut te = 0;
                while ps.is_byte_at(b'\n', ps.ptr - te - 1) {
                    te += 1;
                }
                err.slice = Some((last_entry_end, ps.ptr - te));
                errors.push(err);
                if te > 1 {
                    let slice = ps.get_slice(last_entry_end, ps.ptr - te + 1);
                    body.push(ast::ResourceEntry::Junk(slice));
                    last_entry_end = ps.ptr;
                }
            }
        }
    }
    if let Some(content) = last_comment {
        body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(
            ast::Comment::Comment { content },
        )));
    }
    if last_entry_end != ps.ptr {
        let mut te = 0;
        while ps.is_byte_at(b'\n', ps.ptr - te - 1) {
            te += 1;
        }
        let te = if te > 1 { te - 1 } else { 0 };
        let slice = ps.get_slice(last_entry_end, ps.ptr - te);
        body.push(ast::ResourceEntry::Junk(slice));
    }

    if errors.is_empty() {
        Ok(ast::Resource { body })
    } else {
        Err((ast::Resource { body }, errors))
    }
}

fn get_entry<'p>(ps: &mut ParserStream<'p>) -> Result<ast::Entry<'p>> {
    let entry = match ps.source[ps.ptr] {
        b'#' => ast::Entry::Comment(get_comment(ps)?),
        b'-' => ast::Entry::Term(get_term(ps)?),
        _ => ast::Entry::Message(get_message(ps)?),
    };
    Ok(entry)
}

fn get_message<'p>(ps: &mut ParserStream<'p>) -> Result<ast::Message<'p>> {
    let id = get_identifier(ps)?;
    ps.skip_blank_inline();
    ps.expect_byte(b'=')?;
    ps.skip_blank_inline();

    let pattern = if ps.skip_to_value_start() {
        get_pattern(ps)?
    } else {
        None
    };

    ps.skip_blank_block();

    let ptr = ps.ptr;
    let attributes = match get_attributes(ps) {
        Ok(attrs) => attrs,
        Err(_err) => {
            ps.ptr = ptr;
            vec![]
        }
    };

    if pattern.is_none() && attributes.is_empty() {
        return error!(
            ps,
            ErrorKind::ExpectedMessageField {
                entry_id: id.name.to_string()
            }
        );
    }

    Ok(ast::Message {
        id,
        value: pattern,
        attributes,
        comment: None,
    })
}

fn get_term<'p>(ps: &mut ParserStream<'p>) -> Result<ast::Term<'p>> {
    ps.expect_byte(b'-')?;
    let id = get_identifier(ps)?;
    ps.skip_blank_inline();
    ps.expect_byte(b'=')?;
    ps.skip_blank_inline();

    let value = get_value(ps)?;

    ps.skip_blank_block();

    let ptr = ps.ptr;
    let attributes = match get_attributes(ps) {
        Ok(attrs) => attrs,
        Err(_err) => {
            ps.ptr = ptr;
            vec![]
        }
    };

    if let Some(value) = value {
        Ok(ast::Term {
            id,
            value,
            attributes,
            comment: None,
        })
    } else {
        error!(
            ps,
            ErrorKind::ExpectedTermField {
                entry_id: id.name.to_string()
            }
        )
    }
}

fn get_value<'p>(ps: &mut ParserStream<'p>) -> Result<Option<ast::Value<'p>>> {
    if !ps.skip_to_value_start() {
        return Ok(None);
    }

    if ps.is_current_byte(b'{') {
        let start = ps.ptr;
        ps.ptr += 1;
        ps.skip_blank();
        if ps.is_current_byte(b'*') || ps.is_current_byte(b'[') {
            let variants = get_variants(ps, true)?;
            ps.expect_byte(b'}')?;
            return Ok(Some(ast::Value::VariantList { variants }));
        }
        ps.ptr = start;
    }

    let pattern = get_pattern(ps)?;

    Ok(pattern.map(ast::Value::Pattern))
}

fn get_attributes<'p>(ps: &mut ParserStream<'p>) -> Result<Vec<ast::Attribute<'p>>> {
    let mut attributes = vec![];

    loop {
        let line_start = ps.ptr;

        ps.skip_blank_inline();

        if !ps.is_current_byte(b'.') {
            ps.ptr = line_start;
            break;
        }
        ps.ptr += 1; // .
        let id = get_identifier(ps)?;
        ps.skip_blank_inline();
        ps.expect_byte(b'=')?;
        ps.skip_blank_inline();
        let pattern = get_pattern(ps)?;

        match pattern {
            Some(pattern) => attributes.push(ast::Attribute { id, value: pattern }),
            None => panic!("Expected Value!"),
        };
        ps.skip_eol();
    }
    Ok(attributes)
}

fn get_identifier<'p>(ps: &mut ParserStream<'p>) -> Result<ast::Identifier<'p>> {
    let start_pos = ps.ptr;

    while ps.ptr < ps.length {
        let b = ps.source[ps.ptr];
        if start_pos == ps.ptr {
            if ps.is_identifier_start() {
                ps.ptr += 1;
            } else {
                return error!(
                    ps,
                    ErrorKind::ExpectedCharRange {
                        range: "a-zA-Z".to_string()
                    }
                );
            }
        } else if (b >= b'a' && b <= b'z')
            || (b >= b'A' && b <= b'Z')
            || (b >= b'0' && b <= b'9')
            || b == b'_'
            || b == b'-'
        {
            ps.ptr += 1;
        } else {
            break;
        }
    }
    let name = ps.get_slice(start_pos, ps.ptr);

    Ok(ast::Identifier { name })
}

fn get_variant_key<'p>(ps: &mut ParserStream<'p>) -> Result<ast::VariantKey<'p>> {
    if !ps.take_if(b'[') {
        return error!(ps, ErrorKind::ExpectedToken('['));
    }
    ps.skip_blank();

    let key = if ps.is_number_start() {
        ast::VariantKey::NumberLiteral {
            value: get_number_literal(ps)?,
        }
    } else {
        ast::VariantKey::Identifier {
            name: get_identifier(ps)?.name,
        }
    };

    ps.skip_blank();

    ps.expect_byte(b']')?;

    Ok(key)
}

fn get_variants<'p>(
    ps: &mut ParserStream<'p>,
    variant_lists: bool,
) -> Result<Vec<ast::Variant<'p>>> {
    let mut variants = vec![];
    let mut has_default = false;

    while ps.is_current_byte(b'*') || ps.is_current_byte(b'[') {
        let default = ps.take_if(b'*');

        if default {
            if has_default {
                return error!(ps, ErrorKind::MultipleDefaultVariants);
            } else {
                has_default = true;
            }
        }

        let key = get_variant_key(ps)?;

        ps.skip_blank_inline();

        let value = if variant_lists {
            get_value(ps)?
        } else {
            get_pattern(ps)?.map(ast::Value::Pattern)
        };

        if let Some(value) = value {
            variants.push(ast::Variant {
                key,
                value,
                default,
            });
            ps.skip_blank();
        } else {
            return error!(ps, ErrorKind::MissingValue);
        }
    }

    if !has_default {
        error!(ps, ErrorKind::MissingDefaultVariant)
    } else {
        Ok(variants)
    }
}

fn get_pattern<'p>(ps: &mut ParserStream<'p>) -> Result<Option<ast::Pattern<'p>>> {
    let start = ps.ptr;
    if ps.skip_eol() {
        ps.skip_blank_block();
        if !ps.skip_blank_inline() || !ps.is_pattern_start() {
            ps.ptr = start;
            return Ok(None);
        }
    }

    let mut elements = vec![];

    loop {
        let mut start_pos = ps.ptr;

        while ps.ptr < ps.length {
            if ps.skip_eol() {
                break;
            }
            let b = ps.source[ps.ptr];
            match b {
                b'\\' => {
                    ps.ptr += 1;
                    let b = ps.source[ps.ptr];
                    match b {
                        b'{' => ps.ptr += 1,
                        b'\\' => ps.ptr += 1,
                        b'u' => {
                            ps.ptr += 2;
                            let start = ps.ptr;
                            for _ in 0..4 {
                                match ps.source[ps.ptr] {
                                    b'0'...b'9' => ps.ptr += 1,
                                    b'a'...b'f' => ps.ptr += 1,
                                    b'A'...b'F' => ps.ptr += 1,
                                    _ => break,
                                }
                            }
                            if start == ps.ptr {
                                return error!(
                                    ps,
                                    ErrorKind::InvalidUnicodeEscapeSequence(
                                        ps.get_slice(start, ps.ptr + 1).to_owned()
                                    )
                                );
                            }
                        }
                        _ => panic!(),
                    }
                }
                b'{' => {
                    if start_pos != ps.ptr {
                        let value = ps.get_slice(start_pos, ps.ptr);
                        elements.push(ast::PatternElement::TextElement(value));
                    }
                    ps.ptr += 1; // {
                    ps.skip_blank();
                    let exp = get_expression(ps)?;
                    elements.push(ast::PatternElement::Placeable(exp));
                    ps.skip_blank_inline();
                    ps.expect_byte(b'}')?;
                    start_pos = ps.ptr;
                }
                _ => ps.ptr += 1,
            }
        }

        if start_pos != ps.ptr {
            let value = ps.get_slice(start_pos, ps.ptr);
            elements.push(ast::PatternElement::TextElement(value));
        }

        let end_of_line = ps.ptr;

        let bl = ps.skip_blank_block();

        if !ps.skip_blank_inline() || !ps.is_pattern_start() {
            ps.ptr = end_of_line;
            break;
        } else {
            for _ in 0..bl {
                elements.push(ast::PatternElement::TextElement("\n"));
            }
        }
    }

    if !elements.is_empty() {
        let last_pos = elements.len() - 1;
        let val = &elements[last_pos];
        let mut new_val = "";
        let mut modified = false;
        if let ast::PatternElement::TextElement(te) = val {
            new_val = te.trim_right();
            modified = &new_val != te;
        }
        if modified {
            elements.pop();
            if !new_val.is_empty() {
                elements.insert(last_pos, ast::PatternElement::TextElement(new_val));
                ps.ptr -= 1; // move before last \n
            }
        }
    }

    if elements.is_empty() {
        Ok(None)
    } else {
        Ok(Some(ast::Pattern { elements }))
    }
}

fn get_comment<'p>(ps: &mut ParserStream<'p>) -> Result<ast::Comment<'p>> {
    let mut level = None;
    let mut content = vec![];

    while ps.ptr < ps.length {
        let line_level = get_comment_level(ps);
        if line_level == 0 {
            ps.ptr -= 1;
            break;
        }
        if level.is_some() && Some(line_level) != level {
            ps.ptr -= line_level;
            break;
        }

        level = Some(line_level);

        if ps.is_current_byte(b'\n') {
            content.push(get_comment_line(ps)?);
        } else {
            ps.expect_byte(b' ')?;
            content.push(get_comment_line(ps)?);
        }
        ps.skip_eol();
    }

    let comment = if level == Some(3) {
        ast::Comment::ResourceComment { content }
    } else if level == Some(2) {
        ast::Comment::GroupComment { content }
    } else {
        ast::Comment::Comment { content }
    };
    Ok(comment)
}

fn get_comment_level<'p>(ps: &mut ParserStream<'p>) -> usize {
    let mut chars = 0;

    while ps.take_if(b'#') {
        chars += 1;
    }

    chars
}

fn get_comment_line<'p>(ps: &mut ParserStream<'p>) -> Result<&'p str> {
    let start_pos = ps.ptr;

    while ps.ptr < ps.length && !ps.is_eol() {
        ps.ptr += 1;
    }

    Ok(str::from_utf8(&ps.source[start_pos..ps.ptr]).unwrap())
}

fn get_expression<'p>(ps: &mut ParserStream<'p>) -> Result<ast::Expression<'p>> {
    let exp = get_inline_expression(ps)?;

    ps.skip_blank();

    if !ps.is_current_byte(b'-') || !ps.is_byte_at(b'>', ps.ptr + 1) {
        if let ast::InlineExpression::AttributeExpression { ref reference, .. } = exp {
            if let box ast::InlineExpression::TermReference { .. } = reference {
                return error!(ps, ErrorKind::TermAttributeAsPlaceable);
            }
        }
        return Ok(ast::Expression::InlineExpression(exp));
    }

    match exp {
        ast::InlineExpression::MessageReference { .. } => {
            return error!(ps, ErrorKind::MessageReferenceAsSelector);
        }
        ast::InlineExpression::AttributeExpression { ref reference, .. } => {
            if let box ast::InlineExpression::MessageReference { .. } = reference {
                return error!(ps, ErrorKind::MessageAttributeAsSelector);
            }
        }
        ast::InlineExpression::VariantExpression { .. } => {
            return error!(ps, ErrorKind::VariantAsSelector);
        }
        _ => {}
    }

    ps.ptr += 2; // ->

    ps.skip_blank_inline();
    ps.expect_byte(b'\n')?;
    ps.skip_blank();

    let variants = get_variants(ps, false)?;

    Ok(ast::Expression::SelectExpression {
        selector: exp,
        variants,
    })
}

fn get_inline_expression<'p>(ps: &mut ParserStream<'p>) -> Result<ast::InlineExpression<'p>> {
    match ps.source.get(ps.ptr) {
        Some(b'"') => {
            ps.ptr += 1; // "
            let start = ps.ptr;
            while ps.ptr < ps.length {
                match ps.source[ps.ptr] {
                    b'\\' => match ps.source[ps.ptr + 1] {
                        b'\\' => ps.ptr += 2,
                        b'{' => ps.ptr += 2,
                        b'"' => ps.ptr += 2,
                        b'u' => {
                            ps.ptr += 2;
                            let start = ps.ptr;
                            for _ in 0..4 {
                                match ps.source[ps.ptr] {
                                    b'0'...b'9' => ps.ptr += 1,
                                    b'a'...b'f' => ps.ptr += 1,
                                    b'A'...b'F' => ps.ptr += 1,
                                    _ => break,
                                }
                            }
                            if start == ps.ptr {
                                return error!(
                                    ps,
                                    ErrorKind::InvalidUnicodeEscapeSequence(
                                        ps.get_slice(start, ps.ptr + 1).to_owned()
                                    )
                                );
                            }
                        }
                        _ => panic!(),
                    },
                    b'"' => {
                        break;
                    }
                    _ => ps.ptr += 1,
                }
            }

            ps.expect_byte(b'"')?;
            Ok(ast::InlineExpression::StringLiteral {
                value: ps.get_slice(start, ps.ptr - 1),
            })
        }
        Some(b'0'...b'9') => {
            let num = get_number_literal(ps)?;
            Ok(ast::InlineExpression::NumberLiteral { value: num })
        }
        Some(b'-') => {
            ps.ptr += 1; // -
            if ps.is_identifier_start() {
                let id = get_identifier(ps)?;
                match ps.source[ps.ptr] {
                    b'.' => {
                        ps.ptr += 1; // .
                        let attr = get_identifier(ps)?;
                        Ok(ast::InlineExpression::AttributeExpression {
                            reference: Box::new(ast::InlineExpression::TermReference { id }),
                            name: attr,
                        })
                    }
                    b'[' => {
                        let key = get_variant_key(ps)?;
                        Ok(ast::InlineExpression::VariantExpression {
                            reference: Box::new(ast::InlineExpression::TermReference { id }),
                            key,
                        })
                    }
                    _ => Ok(ast::InlineExpression::TermReference { id }),
                }
            } else {
                ps.ptr -= 1;
                let num = get_number_literal(ps)?;
                Ok(ast::InlineExpression::NumberLiteral { value: num })
            }
        }
        Some(b'$') => {
            ps.ptr += 1; // -
            let id = get_identifier(ps)?;
            Ok(ast::InlineExpression::VariableReference { id })
        }
        Some(b'a'...b'z') | Some(b'A'...b'Z') => {
            let id = get_identifier(ps)?;

            match ps.source[ps.ptr] {
                b'(' => get_call_expression(ps, Some(id)),
                b'.' => {
                    ps.ptr += 1; // .
                    let attr = get_identifier(ps)?;
                    Ok(ast::InlineExpression::AttributeExpression {
                        reference: Box::new(ast::InlineExpression::MessageReference { id }),
                        name: attr,
                    })
                }
                _ => Ok(ast::InlineExpression::MessageReference { id }),
            }
        }
        Some(b'{') => {
            ps.ptr += 1; // {
            ps.skip_blank();
            let exp = get_expression(ps)?;
            ps.skip_blank_inline();
            ps.expect_byte(b'}')?;
            Ok(ast::InlineExpression::Placeable {
                expression: Box::new(exp),
            })
        }
        _ => error!(ps, ErrorKind::MissingLiteral),
    }
}

fn get_call_expression<'p>(
    ps: &mut ParserStream<'p>,
    id: Option<ast::Identifier<'p>>,
) -> Result<ast::InlineExpression<'p>> {
    let id = match id {
        Some(id) => id,
        None => get_identifier(ps)?,
    };
    let (positional, named) = get_call_args(ps)?;
    Ok(ast::InlineExpression::CallExpression {
        callee: ast::Function { name: id.name },
        positional,
        named,
    })
}

fn get_call_args<'p>(
    ps: &mut ParserStream<'p>,
) -> Result<(Vec<ast::InlineExpression<'p>>, Vec<ast::NamedArgument<'p>>)> {
    let mut positional = vec![];
    let mut named = vec![];
    let mut argument_names = vec![];

    ps.expect_byte(b'(')?;
    ps.skip_blank();

    while ps.ptr < ps.length {
        let b = ps.source[ps.ptr];
        if b == b')' {
            break;
        }
        let id = if ps.is_identifier_start() {
            Some(get_identifier(ps)?)
        } else {
            None
        };

        if let Some(id) = id {
            ps.skip_blank();
            if ps.is_current_byte(b':') {
                if argument_names.contains(&id.name.to_owned()) {
                    return error!(ps, ErrorKind::DuplicatedNamedArgument(id.name.to_owned()));
                }
                ps.ptr += 1;
                ps.skip_blank();
                let val = get_inline_expression(ps)?;
                argument_names.push(id.name.to_owned());
                named.push(ast::NamedArgument {
                    name: id,
                    value: val,
                });
            } else if ps.is_current_byte(b'(') {
                positional.push(get_call_expression(ps, Some(id))?);
            } else {
                if !argument_names.is_empty() {
                    return error!(ps, ErrorKind::PositionalArgumentFollowsNamed);
                }
                positional.push(ast::InlineExpression::MessageReference { id });
            }
        } else {
            if !argument_names.is_empty() {
                return error!(ps, ErrorKind::PositionalArgumentFollowsNamed);
            }
            positional.push(get_inline_expression(ps)?);
        }

        ps.skip_blank();
        ps.take_if(b',');
        ps.skip_blank();
    }

    ps.expect_byte(b')')?;
    Ok((positional, named))
}

fn get_number_literal<'p>(ps: &mut ParserStream<'p>) -> Result<&'p str> {
    let start = ps.ptr;
    ps.take_if(b'-');
    while ps.source[ps.ptr] >= b'0' && ps.source[ps.ptr] <= b'9' {
        ps.ptr += 1;
    }
    ps.take_if(b'.');
    while ps.source[ps.ptr] >= b'0' && ps.source[ps.ptr] <= b'9' {
        ps.ptr += 1;
    }

    Ok(ps.get_slice(start, ps.ptr))
}
