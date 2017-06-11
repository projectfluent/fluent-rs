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
    let mut comment = None;

    let mut ps = ParserStream::new(source.chars());

    ps.skip_ws_lines();

    let mut entries = vec![];

    while ps.current().is_some() {
        let entry_start_pos = ps.get_index();

        match get_entry(&mut ps) {
            Ok(entry) => {
                if entry_start_pos == 0 {
                    match entry {
                        ast::Entry::Comment(c) => {
                            comment = Some(c);
                        }
                        _ => {
                            entries.push(entry);
                        }
                    }
                } else {
                    entries.push(entry);
                }
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

    if errors.len() > 0 {
        Err((ast::Resource {
                 body: entries,
                 comment: comment,
             },
             errors))
    } else {
        Ok(ast::Resource {
               body: entries,
               comment: comment,
           })
    }
}

fn get_entry<I>(ps: &mut ParserStream<I>) -> Result<ast::Entry>
    where I: Iterator<Item = char>
{
    let mut comment: Option<ast::Comment> = None;

    if ps.current_is('/') {
        comment = Some(get_comment(ps)?);
    }

    if ps.current_is('[') {
        return Ok(get_section(ps, comment)?);
    }

    if ps.is_id_start() {
        return Ok(get_message(ps, comment)?);
    }

    match comment {
        Some(comment) => Ok(ast::Entry::Comment(comment)),
        None => error!(ErrorKind::ExpectedEntry),
    }
}

fn get_comment<I>(ps: &mut ParserStream<I>) -> Result<ast::Comment>
    where I: Iterator<Item = char>
{
    ps.expect_char('/')?;
    ps.expect_char('/')?;
    ps.take_char_if(' ');

    let mut content = String::new();

    loop {
        while let Some(ch) = ps.take_char(|x| x != '\n') {
            content.push(ch);
        }

        ps.next();

        match ps.current() {
            Some(ch) => {
                match ch {
                    '/' => {
                        content.push('\n');
                        ps.next();
                        ps.expect_char('/')?;
                        ps.take_char_if(' ');
                    }
                    _ => {
                        break;
                    }
                }
            }
            None => {
                break;
            }
        }
    }

    Ok(ast::Comment { content: content })
}

fn get_section<I>(ps: &mut ParserStream<I>, comment: Option<ast::Comment>) -> Result<ast::Entry>
    where I: Iterator<Item = char>
{
    ps.expect_char('[')?;
    ps.expect_char('[')?;

    ps.skip_line_ws();

    let symb = get_symbol(ps)?;

    ps.skip_line_ws();

    ps.expect_char(']')?;
    ps.expect_char(']')?;

    ps.skip_line_ws();

    ps.expect_char('\n')?;

    Ok(ast::Entry::Section {
           name: symb,
           comment: comment,
       })
}


fn get_message<I>(ps: &mut ParserStream<I>, comment: Option<ast::Comment>) -> Result<ast::Entry>
    where I: Iterator<Item = char>
{
    let id = get_identifier(ps)?;

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

    let tags = if ps.is_peek_next_line_tag_start() {
        Some(get_tags(ps)?)
    } else {
        None
    };

    if pattern.is_none() && attributes.is_none() {
        return error!(ErrorKind::MissingField {
                          entry_id: id.name,
                          fields: vec!["Value", "Attribute"],
                      });
    }

    Ok(ast::Entry::Message {
           id: id,
           value: pattern,
           attributes: attributes,
           tags: tags,
           comment: comment,
       })
}

fn get_identifier<I>(ps: &mut ParserStream<I>) -> Result<ast::Identifier>
    where I: Iterator<Item = char>
{
    let mut name = String::new();

    name.push(ps.take_id_start()?);

    loop {
        match ps.take_id_char() {
            Some(ch) => name.push(ch),
            _ => break,
        }
    }

    Ok(ast::Identifier { name: name })
}

fn get_attributes<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Attribute>>
    where I: Iterator<Item = char>
{
    let mut attributes = vec![];
    loop {
        ps.expect_char('\n')?;
        ps.skip_line_ws();

        ps.expect_char('.')?;

        let key = get_identifier(ps)?;

        ps.skip_line_ws();

        ps.expect_char('=')?;

        ps.skip_line_ws();

        let value = get_pattern(ps)?;

        match value {
            Some(pattern) => {
                attributes.push(ast::Attribute {
                                    id: key,
                                    value: pattern,
                                });
            }
            None => return error!(ErrorKind::ExpectedField { field: String::from("Pattern") }),
        }

        if !ps.is_peek_next_line_attribute_start() {
            break;
        }
    }
    Ok(attributes)
}

fn get_tags<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Tag>>
    where I: Iterator<Item = char>
{
    let mut tags = vec![];
    loop {
        ps.expect_char('\n')?;
        ps.skip_line_ws();

        ps.expect_char('#')?;

        let symbol = get_symbol(ps)?;

        tags.push(ast::Tag { name: symbol });

        if !ps.is_peek_next_line_tag_start() {
            break;
        }
    }
    Ok(tags)
}

fn get_variant_key<I>(ps: &mut ParserStream<I>) -> Result<ast::VarKey>
    where I: Iterator<Item = char>
{
    match ps.current() {
        Some(ch) => {
            match ch {
                '0'...'9' | '-' => {
                    let num = get_number(ps)?;
                    return Ok(ast::VarKey::Number(num));
                }
                _ => {
                    ps.reset_peek();
                    return Ok(ast::VarKey::Symbol(get_symbol(ps)?));
                }
            }
        }
        None => error!(ErrorKind::ExpectedField { field: "Symbol | Number".to_owned() }),
    }
}

fn get_variants<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Variant>>
    where I: Iterator<Item = char>
{
    let mut variants = vec![];
    let mut has_default = false;

    loop {
        let mut default_index = false;

        ps.expect_char('\n')?;
        ps.skip_line_ws();

        if ps.current_is('*') {
            ps.next();
            default_index = true;
            has_default = true;
        }

        ps.expect_char('[')?;

        let key = get_variant_key(ps)?;

        ps.expect_char(']')?;

        ps.skip_line_ws();

        let value = get_pattern(ps)?;

        match value {
            Some(pattern) => {
                variants.push(ast::Variant {
                                  key: key,
                                  value: pattern,
                                  default: default_index,
                              });
            }
            None => return error!(ErrorKind::ExpectedField { field: String::from("Pattern") }),
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

fn get_symbol<I>(ps: &mut ParserStream<I>) -> Result<ast::Symbol>
    where I: Iterator<Item = char>
{
    let mut name = String::new();

    name.push(ps.take_id_start()?);

    loop {
        match ps.take_symb_char() {
            Some(ch) => name.push(ch),
            _ => break,
        }
    }

    while name.ends_with(' ') {
        name.pop();
    }

    Ok(ast::Symbol { name: name })
}

fn get_digits<I>(ps: &mut ParserStream<I>) -> Result<String>
    where I: Iterator<Item = char>
{
    let mut num = String::new();

    match ps.current() {
        Some(ch) => {
            match ch {
                '0'...'9' => {
                    num.push(ch);
                    ps.next();
                }
                _ => return error!(ErrorKind::ExpectedCharRange { range: "0...9".to_owned() }),
            }
        }
        None => return error!(ErrorKind::ExpectedCharRange { range: "0...9".to_owned() }),
    }

    loop {
        match ps.current() {
            Some(ch) => {
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
            None => {
                break;
            }
        }
    }

    Ok(num)
}

fn get_number<I>(ps: &mut ParserStream<I>) -> Result<ast::Number>
    where I: Iterator<Item = char>
{
    let mut num = String::new();

    match ps.current() {
        Some('-') => {
            num.push('-');
            ps.next();
        }
        _ => {}
    }

    num.push_str(&get_digits(ps)?);


    match ps.current() {
        Some('.') => {
            num.push('.');
            ps.next();
            num.push_str(&get_digits(ps)?);
        }
        _ => {}
    }

    Ok(ast::Number { value: num })
}

fn get_pattern<I>(ps: &mut ParserStream<I>) -> Result<Option<ast::Pattern>>
    where I: Iterator<Item = char>
{
    let mut buffer = String::new();
    let mut elements = vec![];
    let mut first_line = true;

    loop {
        match ps.current() {
            Some(ch) => {
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
                    '\\' => {
                        match ps.peek() {
                            Some(ch2) => {
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
                            }
                            None => {
                                ps.reset_peek();
                                buffer.push(ch);
                                break;
                            }
                        }
                    }
                    '{' => {
                        ps.next();

                        ps.skip_line_ws();

                        if !buffer.is_empty() {
                            elements.push(ast::PatternElement::TextElement(buffer));
                        }

                        buffer = String::new();

                        elements.push(ast::PatternElement::Expression(get_expression(ps)?));

                        ps.skip_line_ws();

                        ps.expect_char('}')?;

                        continue;
                    }
                    _ => {
                        buffer.push(ch);
                    }
                }
                ps.next();
            }
            None => break,
        }
    }

    if !buffer.is_empty() {
        elements.push(ast::PatternElement::TextElement(buffer));
    }

    Ok(Some(ast::Pattern { elements: elements }))
}

fn get_expression<I>(ps: &mut ParserStream<I>) -> Result<ast::Expression>
    where I: Iterator<Item = char>
{
    if ps.is_peek_next_line_variant_start() {
        let variants = get_variants(ps)?;

        ps.expect_char('\n')?;

        return Ok(ast::Expression::SelectExpression {
                      expression: None,
                      variants: variants,
                  });
    }

    let selector = get_selector_expression(ps)?;

    ps.skip_line_ws();

    match ps.current() {
        Some('-') => {
            match ps.peek() {
                Some('>') => {
                    ps.next();
                    ps.next();

                    ps.skip_line_ws();

                    let variants = get_variants(ps)?;

                    if variants.len() == 0 {
                        return error!(ErrorKind::MissingVariants);
                    }

                    ps.expect_char('\n')?;

                    return Ok(ast::Expression::SelectExpression {
                                  expression: Some(Box::new(selector)),
                                  variants: variants,
                              });
                }
                _ => return error!(ErrorKind::ExpectedToken { token: '>' }),
            }
        }
        _ => {
            ps.reset_peek();
        }
    }

    Ok(selector)
}

fn get_selector_expression<I>(ps: &mut ParserStream<I>) -> Result<ast::Expression>
    where I: Iterator<Item = char>
{
    let literal = get_literal(ps)?;

    match literal {
        ast::Expression::MessageReference { id } => {
            match ps.ch {
                Some('.') => {
                    ps.next();
                    let attr = get_identifier(ps)?;
                    Ok(ast::Expression::AttributeExpression {
                           id: ast::Identifier { name: id },
                           name: attr,
                       })
                }
                Some('[') => {
                    ps.next();
                    let key = get_variant_key(ps)?;
                    ps.expect_char(']')?;

                    Ok(ast::Expression::VariantExpression {
                           id: ast::Identifier { name: id },
                           key: key,
                       })
                }
                Some('(') => {
                    ps.next();
                    let args = get_call_args(ps)?;
                    ps.expect_char(')')?;
                    Ok(ast::Expression::CallExpression {
                           callee: ast::Function { name: id },
                           args: args,
                       })
                }
                _ => Ok(ast::Expression::MessageReference { id: id }),
            }
        }
        _ => Ok(literal),
    }
}

fn get_call_args<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Argument>>
    where I: Iterator<Item = char>
{
    let mut args = vec![];

    ps.skip_line_ws();

    loop {
        match ps.current() {
            Some(')') => {
                break;
            }
            _ => {
                let exp = get_selector_expression(ps)?;

                ps.skip_line_ws();

                match ps.current() {
                    Some(':') => {
                        match exp {
                            ast::Expression::MessageReference { id } => {
                                ps.next();
                                ps.skip_line_ws();

                                let val = get_arg_val(ps)?;
                                args.push(ast::Argument::NamedArgument {
                                              name: ast::Identifier { name: id },
                                              val: val,
                                          });
                            }
                            _ => {
                                return error!(ErrorKind::ForbiddenKey);
                            }
                        }
                    }
                    _ => {
                        args.push(ast::Argument::Expression(exp));
                    }
                }

                ps.skip_line_ws();

                match ps.current() {
                    Some(',') => {
                        ps.next();
                        ps.skip_line_ws();
                        continue;
                    }
                    Some(')') => {
                        break;
                    }
                    _ => {
                        return error!(ErrorKind::ExpectedCharRange {
                                          range: "\",\" or \"}\"".to_owned(),
                                      });
                    }
                }
            }
        }
    }

    Ok(args)
}

fn get_arg_val<I>(ps: &mut ParserStream<I>) -> Result<ast::ArgValue>
    where I: Iterator<Item = char>
{
    match ps.current() {
        Some(ch) => {
            match ch {
                '0'...'9' | '-' => Ok(ast::ArgValue::Number(get_number(ps)?)),
                '"' => Ok(ast::ArgValue::String(get_string(ps)?)),
                _ => error!(ErrorKind::ExpectedField { field: String::from("Argument value") }),
            }
        }
        None => error!(ErrorKind::ExpectedField { field: String::from("Literal") }),
    }
}

fn get_string<I>(ps: &mut ParserStream<I>) -> Result<String>
    where I: Iterator<Item = char>
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
    where I: Iterator<Item = char>
{

    let exp = match ps.current() {
        Some(ch) => {
            match ch {
                '0'...'9' | '-' => ast::Expression::NumberExpression(get_number(ps)?),
                '$' => {
                    ps.next();
                    ast::Expression::ExternalArgument { id: get_identifier(ps)?.name }
                }
                '"' => {

                    let string = get_string(ps)?;
                    ast::Expression::StringExpression(string)
                }
                '{' => {
                    ps.next();
                    ps.skip_line_ws();
                    let exp = get_expression(ps)?;
                    ps.skip_line_ws();
                    ps.expect_char('}')?;
                    exp
                }
                _ => ast::Expression::MessageReference { id: get_identifier(ps)?.name },
            }
        }
        None => return error!(ErrorKind::ExpectedField { field: String::from("Literal") }),
    };

    Ok(exp)
}

fn get_junk_entry<I>(ps: &mut ParserStream<I>, source: &str, entry_start: usize) -> ast::Entry
    where I: Iterator<Item = char>
{
    ps.skip_to_next_entry_start();

    let slice = get_error_slice(source, entry_start, ps.get_index());

    ast::Entry::Junk { content: String::from(slice) }
}
