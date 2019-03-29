mod helpers;
use fluent_bundle::FluentValue;

use self::helpers::{
    assert_format_no_errors, assert_get_bundle_no_errors, assert_get_resource_from_str_no_errors,
};

#[test]
fn functions_runtime_passing_into_the_constructor() {
    let res = assert_get_resource_from_str_no_errors(
        r#"
foo = { CONCAT("Foo", "Bar") }
bar = { SUM(1, 2) }
                                                     "#,
    );
    let mut bundle = assert_get_bundle_no_errors(&res, None);
    bundle
        .add_function("CONCAT", |args, _named_args| {
            let mut result = String::new();
            for arg in args {
                match arg {
                    FluentValue::String(s) => {
                        result.push_str(s);
                    }
                    _ => unimplemented!(),
                }
            }
            FluentValue::String(result.into())
        })
        .expect("Failed to add a function to the bundle.");
    bundle
        .add_function("SUM", |args, _named_args| {
            let mut result: isize = 0;
            for arg in args {
                match arg {
                    FluentValue::Number(n) => {
                        let part: isize = n.parse().unwrap();
                        result += part;
                    }
                    _ => unimplemented!(),
                }
            }
            FluentValue::Number(result.to_string().into())
        })
        .expect("Failed to add a function to the bundle.");

    assert_format_no_errors(bundle.format("foo", None), "FooBar");

    assert_format_no_errors(bundle.format("bar", None), "3");
}
