pub use super::errors::ParserError;
pub use super::errors::ErrorKind;
pub use super::errors::get_error_slice;
pub use super::errors::get_error_info;

use super::iter::ParserStream;
use super::stream::FTLParserStream;

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
            Ok(entry) => entries.push(entry),
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
        Err((ast::Resource { body: entries }, errors))
    } else {
        Ok(ast::Resource { body: entries })
    }
}

fn get_entry<I>(ps: &mut ParserStream<I>) -> Result<ast::Entry>
    where I: Iterator<Item = char>
{
    let mut comment: Option<ast::Comment> = None;

    if ps.current_is('#') {
        comment = Some(get_comment(ps)?);
    }

    if ps.current_is('[') {
        return Ok(ast::Entry::Section(get_section(ps, comment)?));
    }

    if ps.is_id_start() {
        return Ok(ast::Entry::Message(get_message(ps, comment)?));
    }

    match comment {
        Some(comment) => Ok(ast::Entry::Comment(comment)),
        None => error!(ErrorKind::ExpectedEntry),
    }
}

fn get_comment<I>(ps: &mut ParserStream<I>) -> Result<ast::Comment>
    where I: Iterator<Item = char>
{
    ps.expect_char('#')?;
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
                    '#' => {
                        content.push('\n');
                        ps.next();
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

    Ok(ast::Comment { body: content })
}

fn get_section<I>(ps: &mut ParserStream<I>, comment: Option<ast::Comment>) -> Result<ast::Section>
    where I: Iterator<Item = char>
{
    ps.expect_char('[')?;
    ps.expect_char('[')?;

    ps.skip_line_ws();

    let key = get_key(ps, true, true)?;

    ps.skip_line_ws();

    ps.expect_char(']')?;
    ps.expect_char(']')?;

    ps.skip_line_ws();

    ps.expect_char('\n')?;

    Ok(ast::Section {
        key: key,
        body: vec![],
        comment: comment,
    })
}


fn get_message<I>(ps: &mut ParserStream<I>, comment: Option<ast::Comment>) -> Result<ast::Message>
    where I: Iterator<Item = char>
{
    let id = get_identifier(ps)?;

    ps.skip_line_ws();

    ps.expect_char('=')?;

    ps.skip_line_ws();

    let pattern = get_pattern(ps)?;

    let mut traits: Option<Vec<ast::Member>> = None;

    if ps.is_peek_next_line_member_start() {
        traits = Some(get_members(ps)?);
    };

    if pattern.is_none() && traits.is_none() {
        return error!(ErrorKind::MissingField {
            fields: vec![String::from("Value"), String::from("Traits")],
        });
    }

    Ok(ast::Message {
        id: id,
        value: pattern,
        traits: traits,
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

fn get_member_key<I>(ps: &mut ParserStream<I>) -> Result<ast::MemberKey>
    where I: Iterator<Item = char>
{
    match ps.current() {
        Some(ch) => {
            match ch {
                '0'...'9' | '-' => {
                    let num = get_number(ps)?;
                    return Ok(ast::MemberKey::Number(num));
                }
                _ => {
                    ps.reset_peek();
                    return Ok(ast::MemberKey::Keyword(get_keyword(ps)?));
                }
            }
        }
        None => error!(ErrorKind::ExpectedField { field: "Keyword | Number".to_owned() }),
    }
}

fn get_members<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Member>>
    where I: Iterator<Item = char>
{
    let mut members = vec![];

    loop {
        let mut default_index = false;

        ps.expect_char('\n')?;
        ps.skip_line_ws();

        if ps.current_is('*') {
            ps.next();
            default_index = true;
        }

        ps.expect_char('[')?;

        let key = get_member_key(ps)?;

        ps.expect_char(']')?;

        ps.skip_line_ws();

        let value = get_pattern(ps)?;

        match value {
            Some(pattern) => {
                members.push(ast::Member {
                    key: key,
                    value: pattern,
                    default: default_index,
                });
            }
            None => return error!(ErrorKind::ExpectedField { field: String::from("Pattern") }),
        }

        if !ps.is_peek_next_line_member_start() {
            break;
        }
    }
    Ok(members)
}

fn get_key<I>(ps: &mut ParserStream<I>, start: bool, end_ws: bool) -> Result<ast::Key>
    where I: Iterator<Item = char>
{
    let mut name = String::new();

    if start {
        name.push(ps.take_id_start()?);
    }

    loop {
        match ps.take_kw_char() {
            Some(ch) => name.push(ch),
            _ => break,
        }
    }

    while name.ends_with(' ') {
        if !end_ws {
            return error!(ErrorKind::ForbiddenWhitespace);
        }
        name.pop();
    }

    Ok(ast::Key { name: name })
}

fn get_keyword<I>(ps: &mut ParserStream<I>) -> Result<ast::Keyword>
    where I: Iterator<Item = char>
{
    let ns = get_identifier(ps)?;

    match ps.current() {
        Some('/') => {
            ps.next();
            let key = get_key(ps, true, false)?;

            Ok(ast::Keyword {
                ns: Some(ns),
                name: key,
            })
        }
        Some(']') => {
            Ok(ast::Keyword {
                ns: None,
                name: ast::Key { name: ns.name },
            })
        }
        _ => {
            let key = get_key(ps, false, false)?;

            Ok(ast::Keyword {
                ns: None,
                name: ast::Key { name: ns.name + &key.name },
            })
        }
    }

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
    let mut quote_delimited = false;
    let mut quote_open = false;
    let mut first_line = true;
    let mut is_intended = false;

    if ps.take_char_if('"') {
        quote_delimited = true;
        quote_open = true;
    }

    loop {
        match ps.current() {
            Some(ch) => {
                match ch {
                    '\n' => {
                        if quote_delimited {
                            return error!(ErrorKind::ExpectedToken { token: '"' });
                        }

                        if first_line && !buffer.is_empty() {
                            break;
                        }

                        ps.peek();

                        ps.peek_line_ws();

                        if !ps.current_peek_is('|') {
                            ps.reset_peek();
                            break;
                        } else {
                            ps.skip_to_peek();
                            ps.next();
                        }

                        if first_line {
                            if ps.take_char_if(' ') {
                                is_intended = true;
                            }
                        } else {
                            if is_intended && !ps.take_char_if(' ') {
                                return error!(ErrorKind::Generic);
                            }
                        }

                        first_line = false;

                        if !buffer.is_empty() {
                            buffer.push(ch);
                        }
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
                                    '"' if quote_delimited => {
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

                        if !buffer.is_empty() {
                            elements.push(ast::PatternElement::Text(buffer));
                        }

                        buffer = String::new();

                        elements.push(ast::PatternElement::Placeable {
                            expressions: get_placeable(ps)?,
                        });

                        ps.expect_char('}')?;

                        continue;
                    }
                    '"' if quote_open => {
                        ps.next();
                        quote_open = false;
                        break;
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

    if quote_open {
        return error!(ErrorKind::ExpectedToken { token: '"' });
    }

    if !buffer.is_empty() {
        elements.push(ast::PatternElement::Text(buffer));
    }

    if !quote_delimited && elements.len() == 0 {
        return Ok(None);
    }

    Ok(Some(ast::Pattern {
        elements: elements,
        quoted: quote_delimited,
    }))
}

fn get_placeable<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Expression>>
    where I: Iterator<Item = char>
{
    let mut exprs = vec![];

    ps.skip_line_ws();

    loop {
        exprs.push(get_placeable_expression(ps)?);

        ps.skip_line_ws();

        match ps.current() {
            Some('}') => {
                break;
            }
            Some(',') => {
                ps.next();
                ps.skip_line_ws();
            }
            _ => return error!(ErrorKind::ExpectedCharRange { range: "'}', ','".to_owned() }),
        }
    }

    Ok(exprs)
}

fn get_placeable_expression<I>(ps: &mut ParserStream<I>) -> Result<ast::Expression>
    where I: Iterator<Item = char>
{
    let selector = get_call_expression(ps)?;

    ps.skip_line_ws();

    match ps.current() {
        Some('-') => {
            match ps.peek() {
                Some('>') => {
                    ps.next();
                    ps.next();

                    ps.skip_line_ws();

                    let members = get_members(ps)?;

                    if members.len() == 0 {
                        return error!(ErrorKind::Generic);
                    }

                    ps.expect_char('\n')?;

                    return Ok(ast::Expression::SelectExpression {
                        exp: Box::new(selector),
                        vars: members,
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

fn get_call_expression<I>(ps: &mut ParserStream<I>) -> Result<ast::Expression>
    where I: Iterator<Item = char>
{
    let exp = get_member_expression(ps)?;


    if !ps.take_char_if('(') {
        return Ok(exp);
    }

    match exp {
        ast::Expression::MessageReference { id } => {

            let args = get_call_args(ps)?;

            ps.expect_char(')')?;
            return Ok(ast::Expression::CallExpression {
                callee: id,
                args: args,
            });
        }
        _ => error!(ErrorKind::ForbiddenCallee),
    }
}

fn get_call_args<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Expression>>
    where I: Iterator<Item = char>
{
    let mut args = vec![];

    ps.skip_line_ws();

    loop {
        match ps.current() {
            Some(')') => {
                break;
            }
            Some(',') => {
                ps.next();
                ps.skip_line_ws();
            }
            _ => {
                let exp = get_call_expression(ps)?;

                ps.skip_line_ws();

                match ps.current() {
                    Some(':') => {
                        ps.next();
                        ps.skip_line_ws();

                        let val = get_call_expression(ps)?;

                        match exp {
                            ast::Expression::MessageReference { id } => {
                                args.push(ast::Expression::KeyValueArgument {
                                    name: id,
                                    val: Box::new(val),
                                });
                            }
                            _ => {
                                return error!(ErrorKind::ForbiddenKey);
                            }
                        }
                    }
                    _ => {
                        args.push(exp);
                    }
                }
            }
        }
    }

    Ok(args)
}

fn get_member_expression<I>(ps: &mut ParserStream<I>) -> Result<ast::Expression>
    where I: Iterator<Item = char>
{
    let mut exp = get_literal(ps)?;

    loop {
        match ps.current() {
            Some('[') => {
                ps.next();
                let keyword = get_member_key(ps)?;

                ps.expect_char(']')?;

                exp = ast::Expression::Member {
                    obj: Box::new(exp),
                    key: keyword,
                }
            }
            _ => {
                break;
            }
        }
    }

    Ok(exp)
}


fn get_literal<I>(ps: &mut ParserStream<I>) -> Result<ast::Expression>
    where I: Iterator<Item = char>
{

    let exp = match ps.current() {
        Some(ch) => {
            match ch {
                '0'...'9' | '-' => ast::Expression::Number(get_number(ps)?),
                '"' => {
                    match get_pattern(ps)? {
                        Some(ref p) if p.elements.len() == 1 => {
                            match p.elements[0] {
                                ast::PatternElement::Text(ref t) => {
                                    ast::Expression::String(t.clone())
                                }
                                _ => {
                                    return error!(ErrorKind::ExpectedField {
                                        field: String::from("String"),
                                    })
                                }
                            }
                        }
                        Some(_) => {
                            return error!(ErrorKind::ExpectedField {
                                field: String::from("String"),
                            });
                        }
                        None => {
                            return error!(ErrorKind::ExpectedField {
                                field: String::from("String"),
                            });
                        }
                    }
                }
                '$' => {
                    ps.next();
                    ast::Expression::ExternalArgument { id: get_identifier(ps)? }
                }
                _ => ast::Expression::MessageReference { id: get_identifier(ps)? },
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

    ast::Entry::Junk(ast::JunkEntry { body: String::from(slice) })
}
