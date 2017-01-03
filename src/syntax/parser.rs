extern crate itertools;

use std::str;
use std::iter::Iterator;
use std::result;
use self::itertools::{MultiPeek, multipeek};

use super::ast;

#[derive(Debug)]
pub enum ParserError {
    Generic,
}

type Result<T> = result::Result<T, ParserError>;

trait ParserStream<I> {
    fn skip_line_ws(&mut self);
    fn skip_ws(&mut self);
    fn expect_char(&mut self, ch: char) -> Result<()>;
    fn take_char_if(&mut self, ch: char) -> bool;
    fn take_char_after_line_ws_if(&mut self, ch: char) -> bool;
    fn take_char<F>(&mut self, f: F) -> Option<char> where F: Fn(char) -> bool;
    fn take_id_start(&mut self) -> Option<char>;
    fn take_id_char(&mut self) -> Option<char>;
}

impl<I> ParserStream<I> for MultiPeek<I>
    where I: Iterator<Item = char>
{
    fn skip_line_ws(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch != ' ' && ch != '\t' {
                self.reset_peek();
                break;
            }

            self.next();
        }
    }

    fn skip_ws(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch != ' ' && ch != '\n' && ch != '\t' && ch != '\r' {
                self.reset_peek();
                break;
            }

            self.next();
        }
    }

    fn expect_char(&mut self, ch: char) -> Result<()> {
        match self.next() {
            Some(ch2) if ch == ch2 => Ok(()),
            _ => Err(ParserError::Generic),
        }
    }

    fn take_char_if(&mut self, ch: char) -> bool {
        match self.peek() {
            Some(&ch2) if ch == ch2 => {
                self.next();
                true
            }
            _ => {
                self.reset_peek();
                false
            }
        }
    }

    fn take_char_after_line_ws_if(&mut self, ch2: char) -> bool {
        let mut i = 0;

        while let Some(&ch) = self.peek() {
            if ch != ' ' && ch != '\t' {
                if ch == ch2 {
                    i += 1;
                    for _ in 0..i {
                        self.next();
                    }
                    return true;
                } else {
                    self.reset_peek();
                    return false;
                }
            }

            i += 1;
        }

        self.reset_peek();
        return false;
    }

    fn take_char<F>(&mut self, f: F) -> Option<char>
        where F: Fn(char) -> bool
    {

        match self.peek() {
            Some(&ch) if f(ch) => {
                self.next();
                Some(ch)
            }
            _ => {
                self.reset_peek();
                None
            }
        }
    }

    fn take_id_start(&mut self) -> Option<char> {
        let closure = |x| match x {
            'a'...'z' | 'A'...'Z' | '_' => true,
            _ => false,
        };

        match self.take_char(closure) {
            Some(ch) => Some(ch),
            None => None,
        }
    }

    fn take_id_char(&mut self) -> Option<char> {
        let closure = |x| match x {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' | '-' => true,
            _ => false,
        };

        match self.take_char(closure) {
            Some(ch) => Some(ch),
            None => None,
        }
    }
}


pub fn parse(source: &str) -> Result<ast::Resource> {
    let mut ps = multipeek(source.chars());

    let mut entries = vec![];

    while ps.peek().is_some() {
        ps.reset_peek();

        let message = get_entity(&mut ps)?;
        entries.push(ast::Entry::Message(message));

        ps.skip_ws();
    }

    let res = ast::Resource { body: entries };
    Ok(res)
}

fn get_entity<I>(ps: &mut MultiPeek<I>) -> Result<ast::Message>
    where I: Iterator<Item = char>
{
    let id = get_identifier(ps)?;

    ps.skip_line_ws();

    ps.expect_char('=')?;

    ps.skip_line_ws();

    let pattern = get_pattern(ps)?;

    Ok(ast::Message {
        id: id,
        value: Some(pattern),
        traits: None,
    })
}

fn get_identifier<I>(ps: &mut MultiPeek<I>) -> Result<ast::Identifier>
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

fn get_pattern<I>(ps: &mut MultiPeek<I>) -> Result<ast::Pattern>
    where I: Iterator<Item = char>
{
    let mut buffer = String::new();
    let mut elements = vec![];
    let mut quote_delimited = false;
    let mut first_line = true;
    let mut is_intended = false;

    if ps.take_char_if('"') {
        quote_delimited = true;
    }

    loop {
        match ps.peek() {
            Some(&ch) => {
                match ch {
                    '\n' => {
                        ps.next();
                        if quote_delimited {
                            return Err(ParserError::Generic);
                        }

                        if !ps.take_char_after_line_ws_if('|') {
                            break;
                        }

                        if first_line && !buffer.is_empty() {
                            return Err(ParserError::Generic);
                        }


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
                        ps.skip_line_ws();

                        elements.push(ast::PatternElement::Text(buffer));

                        buffer = String::new();

                        elements.push(ast::PatternElement::Placeable {
                            expressions: get_placeable(ps)?,
                        });

                        ps.skip_line_ws();

                        ps.expect_char('}')?;

                        continue;
                    }
                    '"' if quote_delimited => {
                        quote_delimited = false;
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

    if quote_delimited {
        return Err(ParserError::Generic);
    }

    if buffer.len() != 0 {
        elements.push(ast::PatternElement::Text(buffer));
    }

    Ok(ast::Pattern {
        elements: elements,
        quoted: false,
    })
}

fn get_placeable<I>(ps: &mut MultiPeek<I>) -> Result<Vec<ast::Expression>>
    where I: Iterator<Item = char>
{
    let mut placeable = vec![];
    let exp = get_call_expression(ps)?;
    placeable.push(exp);

    Ok(placeable)
}

fn get_call_expression<I>(ps: &mut MultiPeek<I>) -> Result<ast::Expression>
    where I: Iterator<Item = char>
{
    let exp = get_expression(ps)?;

    if !ps.take_char_if('(') {
        return Ok(exp);
    }

    match exp {
        ast::Expression::EntityReference { id } => {
            ps.expect_char(')')?;
            return Ok(ast::Expression::CallExpression {
                callee: id,
                args: vec![],
            });
        }
        _ => Err(ParserError::Generic),
    }
}

fn get_expression<I>(ps: &mut MultiPeek<I>) -> Result<ast::Expression>
    where I: Iterator<Item = char>
{

    let exp = match ps.peek() {
        Some(&'$') => {
            ps.next();
            ast::Expression::ExternalArgument { id: get_identifier(ps)? }
        }
        _ => {
            ps.reset_peek();
            ast::Expression::EntityReference { id: get_identifier(ps)? }
        }
    };

    Ok(exp)
}
