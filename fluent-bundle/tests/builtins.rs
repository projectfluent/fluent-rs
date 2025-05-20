use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use fluent_syntax::ast::Pattern;

#[test]
fn test_builtin_number() {
    // 1. Create bundle
    // typos: ignore start
    let ftl_string = String::from(
        r#"
count = { NUMBER($num, type: "cardinal") ->
   *[other] A
    [one] B
}
order = { NUMBER($num, type: "ordinal") ->
   *[other] {$num}th
    [one] {$num}st
    [two] {$num}nd
    [few] {$num}rd
}
        "#,
        // typos: ignore end
    );

    let mut bundle = FluentBundle::default();
    bundle
        .add_resource(FluentResource::try_new(ftl_string).expect("Could not parse an FTL string."))
        .expect("Failed to add FTL resources to the bundle.");
    bundle
        .add_builtins()
        .expect("Failed to add builtin functions to the bundle.");

    let get_val = |pattern: &Pattern<&'_ str>, num: isize| {
        let mut args = FluentArgs::new();
        args.set("num", FluentValue::from(num));
        let mut errors = vec![];
        let val = bundle.format_pattern(pattern, Some(&args), &mut errors);
        if errors.is_empty() {
            Ok(val.into_owned())
        } else {
            Err(errors)
        }
    };

    let count = bundle
        .get_message("count")
        .expect("Message doesn't exist")
        .value()
        .expect("Message has no value");

    assert_eq!(get_val(count, 0).unwrap(), "A");
    assert_eq!(get_val(count, 1).unwrap(), "B");
    assert_eq!(get_val(count, 2).unwrap(), "A");
    assert_eq!(get_val(count, 12).unwrap(), "A");
    assert_eq!(get_val(count, 15).unwrap(), "A");
    assert_eq!(get_val(count, 123).unwrap(), "A");

    let order = bundle
        .get_message("order")
        .expect("Message doesn't exist")
        .value()
        .expect("Message has no value");

    assert_eq!(get_val(order, 0).unwrap(), "\u{2068}0\u{2069}th");
    assert_eq!(get_val(order, 1).unwrap(), "\u{2068}1\u{2069}st");
    assert_eq!(get_val(order, 2).unwrap(), "\u{2068}2\u{2069}nd");
    assert_eq!(get_val(order, 12).unwrap(), "\u{2068}12\u{2069}th");
    assert_eq!(get_val(order, 15).unwrap(), "\u{2068}15\u{2069}th");
    assert_eq!(get_val(order, 123).unwrap(), "\u{2068}123\u{2069}rd");
}
