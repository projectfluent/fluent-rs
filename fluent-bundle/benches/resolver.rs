use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

use fluent_bundle::{FluentBundle, FluentResource, FluentValue};
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

fn get_args(name: &str) -> Option<HashMap<String, FluentValue>> {
    match name {
        "preferences" => {
            let mut prefs_args = HashMap::new();
            prefs_args.insert("name".to_string(), FluentValue::from("John"));
            prefs_args.insert("tabCount".to_string(), FluentValue::from(5));
            prefs_args.insert("count".to_string(), FluentValue::from(3));
            prefs_args.insert("version".to_string(), FluentValue::from("65.0"));
            prefs_args.insert("path".to_string(), FluentValue::from("/tmp"));
            prefs_args.insert("num".to_string(), FluentValue::from(4));
            prefs_args.insert("email".to_string(), FluentValue::from("john@doe.com"));
            prefs_args.insert("value".to_string(), FluentValue::from(4.5));
            prefs_args.insert("unit".to_string(), FluentValue::from("mb"));
            prefs_args.insert(
                "service-name".to_string(),
                FluentValue::from("Mozilla Disk"),
            );
            Some(prefs_args)
        }
        _ => None,
    }
}

fn add_functions<R>(name: &'static str, bundle: &mut FluentBundle<R>) {
    match name {
        "preferences" => {
            bundle
                .add_function("PLATFORM", |_args, _named_args| {
                    return "linux".into();
                })
                .expect("Failed to add a function to the bundle.");
        }
        _ => {}
    }
}

fn resolver_bench(c: &mut Criterion) {
    let tests = &["simple", "preferences", "menubar", "unescape"];
    let ftl_strings = get_strings(tests);

    c.bench_function_over_inputs(
        "resolve",
        move |b, &&name| {
            let source = &ftl_strings[name];
            let res =
                FluentResource::try_new(source.to_owned()).expect("Couldn't parse an FTL source");
            let ids = get_ids(&res);
            let mut bundle = FluentBundle::new(&["x-testing"]);
            bundle
                .add_resource(res)
                .expect("Couldn't add FluentResource to the FluentBundle");
            add_functions(name, &mut bundle);
            let args = get_args(name);

            b.iter(|| {
                for id in &ids {
                    let msg = bundle.get_message(id).expect("Message found");
                    let mut errors = vec![];
                    if let Some(value) = msg.value {
                        let _ = bundle.format_pattern(value, args.as_ref(), &mut errors);
                    }
                    for (_, value) in msg.attributes {
                        let _ = bundle.format_pattern(value, args.as_ref(), &mut errors);
                    }
                    assert!(errors.len() == 0, "Resolver errors: {:#?}", errors);
                }
            })
        },
        tests,
    );
}

criterion_group!(benches, resolver_bench);
criterion_main!(benches);
