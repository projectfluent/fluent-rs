use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

use fluent_bundle::{FluentBundle, FluentResource};
use fluent_syntax::ast;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn get_strings(tests: &[&'static str]) -> HashMap<&'static str, String> {
    let mut ftl_strings = HashMap::new();
    for test in tests {
        let path = format!("./benches/{}.ftl", test);
        ftl_strings.insert(*test, read_file(&path).expect("Couldn't load file"));
    }
    return ftl_strings;
}

fn get_ids(res: &FluentResource) -> Vec<String> {
    res.ast()
        .body
        .iter()
        .filter_map(|entry| match entry {
            ast::ResourceEntry::Entry(ast::Entry::Message(ast::Message { id, .. })) => {
                Some(id.name.to_owned())
            }
            _ => None,
        })
        .collect()
}

fn resolver_bench(c: &mut Criterion) {
    let tests = &["simple", "menubar", "unescape"];
    let ftl_strings = get_strings(tests);

    c.bench_function_over_inputs(
        "resolve",
        move |b, &&name| {
            let source = &ftl_strings[name];
            let res =
                FluentResource::try_new(source.to_owned()).expect("Couldn't parse an FTL source");
            let ids = get_ids(&res);
            b.iter_with_setup(
                || {
                    let mut bundle = FluentBundle::new(&["x-testing"]);
                    bundle
                        .add_resource(&res)
                        .expect("Couldn't add FluentResource to the FluentBundle");
                    bundle
                },
                |bundle| {
                    for id in &ids {
                        bundle.format(id, None);
                    }
                },
            )
        },
        tests,
    );
}

criterion_group!(benches, resolver_bench);
criterion_main!(benches);
