extern crate itertools;

use std::str;
use std::iter::Iterator;
use std::collections::HashMap;
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
    fn expect_char(&mut self, ch: char) -> Result<()>;
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

    fn expect_char(&mut self, ch: char) -> Result<()> {
        match self.peek() {
            Some(&ch2) if ch == ch2 => Ok(()),
            _ => Err(ParserError::Generic),
        }
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

    let mut entries = HashMap::new();

    let (id, val) = get_entity(&mut ps)?;

    entries.insert(id, val);

    let res = ast::Resource(entries);
    Ok(res)
}

fn get_entity<'a, I>(ps: &mut MultiPeek<I>) -> Result<(String, ast::Value)>
    where I: Iterator<Item = char>
{
    let id = get_identifier(ps)?;

    ps.skip_line_ws();

    ps.expect_char('=')?;

    ps.skip_line_ws();

    let val = get_value(ps)?;

    Ok((id, val))
}

fn get_identifier<'a, I>(ps: &mut MultiPeek<I>) -> Result<String>
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

    Ok(name)
}

fn get_value<'a, I>(ps: &mut MultiPeek<I>) -> Result<ast::Value>
    where I: Iterator<Item = char>
{
    get_pattern(ps)
}

fn get_pattern<'a, I>(ps: &mut MultiPeek<I>) -> Result<ast::Value>
    where I: Iterator<Item = char>
{
    let mut buffer = String::new();

    loop {
        match ps.peek() {
            Some(&ch) => {
                match ch {
                    '\n' => {
                        ps.reset_peek();
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

    Ok(ast::Value::Simple(buffer))
}
