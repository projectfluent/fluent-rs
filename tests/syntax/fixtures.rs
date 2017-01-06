extern crate fluent;
extern crate glob;

use self::glob::glob;
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

fn attempt_parse(source: &str) -> Result<(), ()> {
    match parse(source) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

#[test]
fn parse_ftl() {
    for entry in glob("./tests/fixtures/parser/ftl/*.ftl").expect("Failed to read glob pattern") {

        let p = entry.expect("Error while getting an entry");
        let path = p.to_str().expect("Can't print path");

        if path.contains("errors") {
            continue;
        }

        println!("Attempting to parse file: {}", path);

        let string = read_file(path).expect("Failed to read");

        attempt_parse(&string).expect("Failed to parse");
    }
}

#[test]
fn error_ftl() {
    for entry in glob("./tests/fixtures/parser/ftl/*.ftl").expect("Failed to read glob pattern") {

        let p = entry.expect("Error while getting an entry");
        let path = p.to_str().expect("Can't print path");

        if !path.contains("errors") {
            continue;
        }

        println!("Attempting to parse error file: {}", path);

        let string = read_file(path).expect("Failed to read");

        let chunks = string.split("\n\n");

        for chunk in chunks {
            println!("Testing chunk: {:?}", chunk);
            match attempt_parse(&chunk) {
                Ok(_) => panic!("Test didn't fail"),
                Err(_) => continue,
            }
        }
    }
}
