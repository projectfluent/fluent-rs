extern crate itertools;

use std::str;
use std::iter::Iterator;
use std::collections::HashMap;
use self::itertools::{MultiPeek, multipeek};

use super::ast;

#[derive(Debug)]
pub enum ParserError {
    Generic
}

trait ParserStream<I> {
    fn bump(&mut self);
    fn peek_char(&mut self) -> Option<&char>;
    fn read_char(&mut self) -> Option<char>;
    fn skip_line_ws(&mut self);

}

impl<I> ParserStream<I> for MultiPeek<I>
    where I: Iterator<Item=char>
{
    fn bump(&mut self) {
        self.next();
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.peek()
    }

    fn read_char(&mut self) -> Option<char> {
        self.next()
    }

    fn skip_line_ws(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch != ' ' && ch != '\t' {
                self.reset_peek();
                break;
            }

            self.next();
        }
    }
}


pub fn parse(source: &str) -> Result<ast::Resource, ParserError> {
    let mut ps = multipeek(source.chars());

    let mut entries = HashMap::new();

    let (id, val) = get_entity(&mut ps)?;

    entries.insert(id, val);

    let res = ast::Resource(entries);
    Ok(res)
}

fn get_entity<'a, I>(ps: &mut MultiPeek<I>) -> Result<(String, ast::Value), ParserError>
    where I: Iterator<Item=char>
{
    let id = get_identifier(ps)?;

    ps.skip_line_ws();

    ps.bump();

    ps.skip_line_ws();

    let val = get_value(ps)?;

    Ok((id, val))
}

fn get_identifier<'a, I>(ps: &mut MultiPeek<I>) -> Result<String, ParserError>
    where I: Iterator<Item=char>
{
    let mut name = String::new();

    match ps.read_char() {
        Some(ch) if is_id_start(ch) => name.push(ch),
        _ => panic!(),
    }

    loop {
        match ps.peek_char() {
            Some(&ch) if is_id_char(ch) => name.push(ch),
            _ => break,
        }
        ps.bump();
    }

    ps.reset_peek();

    Ok(name)
}

fn get_value<'a, I>(ps: &mut MultiPeek<I>) -> Result<ast::Value, ParserError>
    where I: Iterator<Item=char>
{
    get_pattern(ps)
}

fn get_pattern<'a, I>(ps: &mut MultiPeek<I>) -> Result<ast::Value, ParserError>
    where I: Iterator<Item=char>
{
    let mut buffer = String::new();

    loop {
        match ps.peek_char() {
            Some(&ch) => {
                match ch {
                    '\n' => {
                        ps.reset_peek();
                        break;
                    },
                    _ => {
                        buffer.push(ch);
                    }
                }
            },
            None => break
        }
    }

    Ok(ast::Value::Simple(buffer))
}

fn is_id_start(ch: char) -> bool {
    match ch {
        'a'...'z' | 'A'...'Z' | '_' => true,
        _ => false,
    }
}

fn is_id_char(ch: char) -> bool {
    match ch {
        'a'...'z' | 'A'...'Z' | '0'...'9' | '_' | '-' => true,
        _ => false,
    }
}
