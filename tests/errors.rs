extern crate fluent;

use fluent::syntax::errors::display::annotate_error;
use fluent::syntax::parser::parse;

#[test]
fn test_annotate_errors() {
    let input = "key Value";

    let res = parse(input);

    match res {
        Ok(_) => panic!("Should have return an error!"),
        Err((_, errors)) => {
            assert_eq!(errors.len(), 1);
            let err = annotate_error(&errors[0], None, false);
            assert_eq!(
                err,
                "error[E0003]: expected token `=`\n  |\n0 | key Value\n  |     ^\n  |"
            );
        }
    }
}
