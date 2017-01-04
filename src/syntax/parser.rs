pub use super::errors::ParserError;

use super::stream::ParserStream;
use super::stream::parserstream;

use std::result;

use super::ast;

type Result<T> = result::Result<T, ParserError>;

pub fn parse(source: &str) -> Result<ast::Resource> {

    let mut ps = parserstream(source.chars());

    println!("{:?}", ps.current());
    println!("{:?}", ps.current_peek());
    println!("{:?}", ps.get_index());
    println!("{:?}", ps.get_peek_index());
    loop {
        match ps.next() {
            Some(ch) => {
                println!("{:?}", ch);
                println!("{:?}", ps.current());
                println!("{:?}", ps.current_peek());
                println!("{:?}", ps.get_index());
                println!("{:?}", ps.get_peek_index());
            }
            None => break,
        }
    }
    println!("{:?}", ps.current());
    println!("{:?}", ps.current_peek());
    println!("{:?}", ps.get_index());
    println!("{:?}", ps.get_peek_index());

    ps.next();

    println!("{:?}", ps.current());
    println!("{:?}", ps.current_peek());
    println!("{:?}", ps.get_index());
    println!("{:?}", ps.get_peek_index());
    // ps.skip_ws_lines();

    Ok(ast::Resource { body: vec![] })
    // get_resource(&mut ps)
}

fn get_resource<I>(ps: &mut ParserStream<I>) -> Result<ast::Resource>
    where I: Iterator<Item = char>
{
    let mut entries = vec![];

    while ps.peek().is_some() {
        ps.reset_peek();

        let entry = get_entry(ps)?;
        entries.push(entry);

        ps.skip_ws();
    }

    Ok(ast::Resource { body: entries })
}

fn get_entry<I>(ps: &mut ParserStream<I>) -> Result<ast::Entry>
    where I: Iterator<Item = char>
{
    let mut comment: Option<ast::Comment> = None;

    match ps.peek() {
        Some(&ch) => {
            match ch {
                '#' => {
                    comment = Some(get_comment(ps)?);
                }
                _ => {
                    ps.reset_peek();
                }
            }
        }
        None => return Err(ParserError::Generic),
    }

    match ps.peek() {
        Some(&ch) => {
            match ch {
                '[' => {
                    return Ok(ast::Entry::Section(get_section(ps, comment)?));
                }
                _ => {
                    ps.reset_peek();
                }
            }
        }
        None => {}
    }

    if ps.is_id_start() {
        return Ok(ast::Entry::Message(get_message(ps, comment)?));
    } else {
        match comment {
            Some(comment) => Ok(ast::Entry::Comment(comment)),
            None => Err(ParserError::Generic),
        }
    }
}

fn get_comment<I>(ps: &mut ParserStream<I>) -> Result<ast::Comment>
    where I: Iterator<Item = char>
{
    ps.next();
    ps.take_char_if(' ');

    let mut content = String::new();

    loop {
        while let Some(ch) = ps.take_char(|x| x != '\n') {
            content.push(ch);
        }

        ps.next();

        match ps.peek() {
            Some(&ch) => {
                match ch {
                    '#' => {
                        content.push('\n');
                        ps.next();
                        ps.take_char_if(' ');
                    }
                    _ => {
                        ps.reset_peek();
                        break;
                    }
                }
            }
            None => {
                ps.reset_peek();
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

    match ps.peek() {
        Some(&ch) => {
            match ch {
                '\n' => {
                    ps.peek_line_ws();

                    match ps.peek() {
                        Some(&'*') => {
                            ps.skip_to_peek();
                            traits = Some(get_members(ps)?);
                        }
                        Some(&'[') => {
                            match ps.peek() {
                                Some(&'[') => {
                                    ps.reset_peek();
                                }
                                _ => {
                                    ps.skip_to_peek();
                                    traits = Some(get_members(ps)?);
                                }
                            }
                        }
                        _ => {
                            ps.reset_peek();
                        }
                    }
                }
                _ => {
                    ps.reset_peek();
                }
            }
        }
        None => {
            ps.reset_peek();
        }
    };

    Ok(ast::Message {
        id: id,
        value: Some(pattern),
        traits: traits,
        comment: comment,
    })
}

fn get_identifier<I>(ps: &mut ParserStream<I>) -> Result<ast::Identifier>
    where I: Iterator<Item = char>
{
    let mut name = String::new();

    match ps.take_id_start() {
        Some(ch) => name.push(ch),
        None => return Err(ParserError::Generic),
    };

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
    match ps.peek() {
        Some(&ch) => {
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
        None => Err(ParserError::Generic),
    }
}

fn get_members<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Member>>
    where I: Iterator<Item = char>
{
    let mut members = vec![];

    loop {
        let mut default_index = false;

        match ps.peek() {
            Some(&'*') => {
                ps.next();
                default_index = true;
            }
            _ => {
                ps.reset_peek();
            }
        };

        match ps.peek() {
            Some(&'[') => {
                match ps.peek() {
                    Some(&'[') => {
                        ps.reset_peek();
                        break;
                    }
                    _ => {
                        ps.reset_peek();
                        ps.next();
                    }
                }

                let key = get_member_key(ps)?;

                ps.expect_char(']')?;

                ps.skip_line_ws();

                let value = get_pattern(ps)?;

                members.push(ast::Member {
                    key: key,
                    value: value,
                    default: default_index,
                });

                match ps.peek() {
                    Some(&'\n') => {
                        ps.next();
                        ps.skip_ws();
                    }
                    _ => {
                        ps.reset_peek();
                        break;
                    }
                }
            }
            _ => {
                ps.reset_peek();
                break;
            }
        }
    }
    Ok(members)
}

fn get_key<I>(ps: &mut ParserStream<I>, start: bool, end_ws: bool) -> Result<ast::Key>
    where I: Iterator<Item = char>
{
    let mut name = String::new();

    if start {
        match ps.take_id_start() {
            Some(ch) => name.push(ch),
            None => return Err(ParserError::Generic),
        };
    }

    loop {
        match ps.take_kw_char() {
            Some(ch) => name.push(ch),
            _ => break,
        }
    }

    while name.ends_with(' ') {
        if !end_ws {
            return Err(ParserError::Generic);
        }
        name.pop();
    }

    Ok(ast::Key { name: name })
}

fn get_keyword<I>(ps: &mut ParserStream<I>) -> Result<ast::Keyword>
    where I: Iterator<Item = char>
{
    let ns = get_identifier(ps)?;

    match ps.peek() {
        Some(&'/') => {
            ps.next();
            let key = get_key(ps, true, false)?;

            Ok(ast::Keyword {
                ns: Some(ns),
                name: key,
            })
        }
        Some(&']') => {
            ps.reset_peek();
            Ok(ast::Keyword {
                ns: None,
                name: ast::Key { name: ns.name },
            })
        }
        _ => {
            ps.reset_peek();
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

    match ps.peek() {
        Some(&ch) => {
            match ch {
                '0'...'9' => {
                    num.push(ch);
                    ps.next();
                }
                _ => return Err(ParserError::Generic),
            }
        }
        None => return Err(ParserError::Generic),
    }

    loop {
        match ps.peek() {
            Some(&ch) => {
                match ch {
                    '0'...'9' => {
                        num.push(ch);
                        ps.next();
                    }
                    _ => {
                        ps.reset_peek();
                        break;
                    }
                }
            }
            None => {
                ps.reset_peek();
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

    match ps.peek() {
        Some(&'-') => {
            num.push('-');
            ps.next();
        }
        _ => {
            ps.reset_peek();
        }
    }

    num.push_str(&get_digits(ps)?);


    match ps.peek() {
        Some(&'.') => {
            num.push('.');
            ps.next();
            num.push_str(&get_digits(ps)?);
        }
        _ => {
            ps.reset_peek();
        }
    }

    Ok(ast::Number { value: num })
}


fn get_pattern<I>(ps: &mut ParserStream<I>) -> Result<ast::Pattern>
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
        match ps.peek() {
            Some(&ch) => {
                match ch {
                    '\n' => {
                        if quote_delimited {
                            return Err(ParserError::Generic);
                        }

                        if first_line && !buffer.is_empty() {
                            ps.reset_peek();
                            break;
                        }

                        if !ps.take_char_after_line_ws_if('|') {
                            break;
                        }

                        ps.next();

                        if first_line {
                            if ps.take_char_if(' ') {
                                is_intended = true;
                            }
                        } else {
                            if is_intended && !ps.take_char_if(' ') {
                                return Err(ParserError::Generic);
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
                            Some(&ch2) => {
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

                        elements.push(ast::PatternElement::Text(buffer));

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
        return Err(ParserError::Generic);
    }

    if buffer.len() != 0 {
        elements.push(ast::PatternElement::Text(buffer));
    }

    Ok(ast::Pattern {
        elements: elements,
        quoted: quote_delimited,
    })
}

fn get_placeable<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Expression>>
    where I: Iterator<Item = char>
{
    let mut exprs = vec![];

    ps.skip_line_ws();

    loop {
        exprs.push(get_placeable_expression(ps)?);

        ps.skip_line_ws();

        match ps.peek() {
            Some(&'}') => {
                ps.reset_peek();
                break;
            }
            Some(&',') => {
                ps.next();
                ps.skip_line_ws();
            }
            _ => return Err(ParserError::Generic),
        }
    }

    Ok(exprs)
}

fn get_placeable_expression<I>(ps: &mut ParserStream<I>) -> Result<ast::Expression>
    where I: Iterator<Item = char>
{
    let selector = get_call_expression(ps)?;

    ps.skip_line_ws();

    match ps.peek() {
        Some(&'-') => {
            match ps.peek() {
                Some(&'>') => {
                    ps.next();
                    ps.next();

                    ps.skip_line_ws();

                    ps.expect_char('\n')?;

                    ps.skip_ws();

                    let members = get_members(ps)?;

                    if members.len() == 0 {
                        return Err(ParserError::Generic);
                    }

                    return Ok(ast::Expression::SelectExpression {
                        exp: Box::new(selector),
                        vars: members,
                    });
                }
                _ => return Err(ParserError::Generic),
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
        _ => Err(ParserError::Generic),
    }
}

fn get_call_args<I>(ps: &mut ParserStream<I>) -> Result<Vec<ast::Expression>>
    where I: Iterator<Item = char>
{
    let mut args = vec![];

    ps.skip_line_ws();

    loop {
        match ps.peek() {
            Some(&')') => {
                ps.reset_peek();
                break;
            }
            Some(&',') => {
                ps.next();
                ps.skip_line_ws();
            }
            _ => {
                ps.reset_peek();

                let exp = get_call_expression(ps)?;

                ps.skip_line_ws();

                match ps.peek() {
                    Some(&':') => {
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
                                return Err(ParserError::Generic);
                            }
                        }
                    }
                    _ => {
                        ps.reset_peek();
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
        match ps.peek() {
            Some(&'[') => {
                ps.next();
                let keyword = get_member_key(ps)?;

                ps.expect_char(']')?;

                exp = ast::Expression::Member {
                    obj: Box::new(exp),
                    key: keyword,
                }
            }
            _ => {
                ps.reset_peek();
                break;
            }
        }
    }

    Ok(exp)
}


fn get_literal<I>(ps: &mut ParserStream<I>) -> Result<ast::Expression>
    where I: Iterator<Item = char>
{

    let exp = match ps.peek() {
        Some(&ch) => {
            match ch {
                '0'...'9' | '-' => {
                    ps.reset_peek();
                    ast::Expression::Number(get_number(ps)?)
                }
                '"' => {
                    ps.reset_peek();

                    let pat = get_pattern(ps)?;

                    if pat.elements.len() == 1 {
                        match pat.elements[0] {
                            ast::PatternElement::Text(ref t) => ast::Expression::String(t.clone()),
                            _ => return Err(ParserError::Generic),
                        }
                    } else {
                        return Err(ParserError::Generic);
                    }
                }
                '$' => {
                    ps.next();
                    ast::Expression::ExternalArgument { id: get_identifier(ps)? }
                }
                _ => {
                    ps.reset_peek();
                    ast::Expression::MessageReference { id: get_identifier(ps)? }
                }
            }
        }
        None => return Err(ParserError::Generic),
    };

    Ok(exp)
}
