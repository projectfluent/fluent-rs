#![feature(test)]

extern crate fluent;
extern crate fluent_syntax;
extern crate test;

use fluent::context::FluentBundle;
use fluent_syntax::{ast, parser::parse};
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
fn bench_simple_format(b: &mut Bencher) {
    let source = read_file("./benches/simple.ftl").expect("Couldn't load file");
    let resource = parse(&source).unwrap();

    let mut ids = Vec::new();

    for entry in resource.body {
        match entry {
            ast::Entry::Message(ast::Message { id, .. }) => ids.push(id.name),
            _ => continue,
        };
    }

    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(&source).unwrap();

    b.iter(|| {
        for id in &ids {
            bundle.format(id.as_str(), None);
        }
    });
}

#[bench]
fn bench_menubar_format(b: &mut Bencher) {
    let source = read_file("./benches/menubar.ftl").expect("Couldn't load file");
    let resource = parse(&source).unwrap();

    let mut ids = Vec::new();

    for entry in resource.body {
        match entry {
            ast::Entry::Message(ast::Message { id, .. }) => ids.push(id.name),
            _ => continue,
        };
    }

    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(&source).unwrap();

    b.iter(|| {
        for id in &ids {
            // In real-life usage we'd have different fallback strategies for missing messages
            // depending on the type of the widget this message was supposed to translate.  Some
            // widgets may only expect attributes and they shouldn't be forced to display a value.
            // Here however it doesn't matter because we know for certain that the message for `id`
            // exists.
            bundle.format_message(id.as_str(), None);
        }
    });
}
