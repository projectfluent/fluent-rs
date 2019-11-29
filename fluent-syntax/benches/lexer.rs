use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

use fluent_syntax::parser2::lexer::Lexer;

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


fn lexer_bench(c: &mut Criterion) {
    let tests = &["simple", "menubar"];
    let ftl_strings = get_strings(tests);

    c.bench_function_over_inputs(
        "lexer_bench",
        move |b, &name| {
            let source = &ftl_strings[name];
            b.iter(|| {
                let parser = Lexer::new(source.as_bytes());
                let ast: Vec<_> = parser.collect();
            });
        },
        tests,
    );
}


criterion_group!(benches, lexer_bench);
criterion_main!(benches);
