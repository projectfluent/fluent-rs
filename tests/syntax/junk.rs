extern crate fluent;

use std::io::prelude::*;
use std::fs::File;
use std::io;

use self::fluent::syntax::parse;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

#[test]
fn basic_junk() {
    let path = "./tests/fixtures/parser/ftl/junk/01-basic.ftl";
    let source = read_file(path).expect("Failed to read");
    match parse(&source) {
        Ok(_) => panic!("Expected junk in the file"),
        Err((res, errors)) => {
            assert_eq!(2, errors.len());
            assert_eq!(8, res.body.len());
        }
    }
}

#[test]
fn start_junk() {
    let path = "./tests/fixtures/parser/ftl/junk/02-start.ftl";
    let source = read_file(path).expect("Failed to read");
    match parse(&source) {
        Ok(_) => panic!("Expected junk in the file"),
        Err((res, errors)) => {
            assert_eq!(1, errors.len());
            assert_eq!(2, res.body.len());
        }
    }
}

#[test]
fn end_junk() {
    let path = "./tests/fixtures/parser/ftl/junk/03-end.ftl";
    let source = read_file(path).expect("Failed to read");
    match parse(&source) {
        Ok(_) => panic!("Expected junk in the file"),
        Err((res, errors)) => {
            assert_eq!(1, errors.len());
            assert_eq!(3, res.body.len());
        }
    }
}

#[test]
fn multiline_junk() {
    let path = "./tests/fixtures/parser/ftl/junk/04-multiline.ftl";
    let source = read_file(path).expect("Failed to read");
    match parse(&source) {
        Ok(_) => panic!("Expected junk in the file"),
        Err((res, errors)) => {
            assert_eq!(1, errors.len());
            assert_eq!(3, res.body.len());
        }
    }
}
