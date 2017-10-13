#![feature(test)]

extern crate test;
extern crate fluent;

use std::io;
use std::io::Read;
use std::fs::File;
use test::Bencher;
use fluent::syntax::parse;


fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}


#[bench]
fn bench_parser_simple(b: &mut Bencher) {
    let f = read_file("./benches/simple.ftl").expect("Couldn't load file");

    b.iter(|| { parse(&f).unwrap(); });
}

#[bench]
fn bench_parser_menubar(b: &mut Bencher) {
    let f = read_file("./benches/menubar.ftl").expect("Couldn't load file");

    b.iter(|| { parse(&f).unwrap(); });
}
