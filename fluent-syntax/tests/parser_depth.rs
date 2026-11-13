use fluent_syntax::ast;
use fluent_syntax::parser::{parse, ErrorKind};

fn parse_on_small_stack(source: String) -> fluent_syntax::parser::Result<String> {
    // A spawned thread gets a small stack on purpose: without a depth limit
    // deeply nested input overflows it and aborts the whole process.
    std::thread::Builder::new()
        .stack_size(2 << 20)
        .spawn(move || parse(source))
        .expect("failed to spawn the parser thread")
        .join()
        .expect("the parser thread aborted")
}

#[test]
fn deeply_nested_placeables_do_not_overflow_the_stack() {
    let n = 3000;
    let source = format!("a = {}{}", "{".repeat(n), "}".repeat(n));

    let (resource, errors) =
        parse_on_small_stack(source).expect_err("the entry should be rejected as junk");

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, ErrorKind::TooDeepNesting);
    assert!(matches!(resource.body[0], ast::Entry::Junk { .. }));
}

#[test]
fn deeply_nested_selects_do_not_overflow_the_stack() {
    let n = 3000;
    let source = format!(
        "a = {}{}",
        "{ $x ->\n   *[o] ".repeat(n),
        "\n}".repeat(n)
    );

    let (resource, errors) =
        parse_on_small_stack(source).expect_err("the entry should be rejected as junk");

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, ErrorKind::TooDeepNesting);
    assert!(matches!(resource.body[0], ast::Entry::Junk { .. }));
}

#[test]
fn deeply_nested_call_arguments_do_not_overflow_the_stack() {
    let n = 3000;
    let source = format!("a = {{ {}{} }}", "NUMBER(".repeat(n), ")".repeat(n));

    let (resource, errors) =
        parse_on_small_stack(source).expect_err("the entry should be rejected as junk");

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, ErrorKind::TooDeepNesting);
    assert!(matches!(resource.body[0], ast::Entry::Junk { .. }));
}

#[test]
fn reasonably_nested_expressions_still_parse() {
    let source = r#"a = { NUMBER({ $count ->
   *[one] { "one" }
    [other] { $count }
}) }
"#;

    let resource = parse(source).expect("reasonably nested input should parse cleanly");
    assert_eq!(resource.body.len(), 1);
}
