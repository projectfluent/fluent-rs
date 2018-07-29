#![feature(test)]

extern crate fluent_syntax;
extern crate test;

use fluent_syntax::parser::parse;
use std::fs::File;
use std::io;
use std::io::Read;
use test::Bencher;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

#[bench]
fn bench_simple_parse(b: &mut Bencher) {
    let source = read_file("./benches/simple.ftl").expect("Couldn't load file");

    b.iter(|| parse(&source).unwrap());
}

#[bench]
fn bench_menubar_parse(b: &mut Bencher) {
    let source = read_file("./benches/menubar.ftl").expect("Couldn't load file");

    b.iter(|| parse(&source).unwrap());
}
