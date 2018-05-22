extern crate fluent;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use self::fluent::syntax::errors::ErrorInfo;
use self::fluent::syntax::errors::ErrorKind;
use self::fluent::syntax::parse;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

#[test]
fn empty_errors() {
    let path = "./tests/fixtures/parser/ftl/errors/01-empty.ftl";
    let source = read_file(path).expect("Failed to read");
    match parse(&source) {
        Ok(_) => panic!("Expected errors in the file"),
        Err((_, ref errors)) => {
            assert_eq!(1, errors.len());

            let error1 = &errors[0];

            assert_eq!(ErrorKind::ExpectedEntry, error1.kind);

            assert_eq!(
                Some(ErrorInfo {
                    slice: " key = value".to_owned(),
                    line: 0,
                    col: 0,
                    pos: 0,
                },),
                error1.info
            );
        }
    }
}

#[test]
fn bad_id_start_errors() {
    let path = "./tests/fixtures/parser/ftl/errors/02-bad-id-start.ftl";
    let source = read_file(path).expect("Failed to read");
    match parse(&source) {
        Ok(_) => panic!("Expected errors in the file"),
        Err((_, ref errors)) => {
            assert_eq!(1, errors.len());

            let error1 = &errors[0];

            assert_eq!(ErrorKind::ExpectedEntry, error1.kind);

            assert_eq!(
                Some(ErrorInfo {
                    slice: "2".to_owned(),
                    line: 0,
                    col: 0,
                    pos: 0,
                },),
                error1.info
            );
        }
    }
}

#[test]
fn just_id_errors() {
    let path = "./tests/fixtures/parser/ftl/errors/03-just-id.ftl";
    let source = read_file(path).expect("Failed to read");
    match parse(&source) {
        Ok(_) => panic!("Expected errors in the file"),
        Err((_, ref errors)) => {
            assert_eq!(1, errors.len());

            let error1 = &errors[0];

            assert_eq!(ErrorKind::ExpectedToken { token: '\u{2424}' }, error1.kind);

            assert_eq!(
                Some(ErrorInfo {
                    slice: "key".to_owned(),
                    line: 0,
                    col: 3,
                    pos: 3,
                },),
                error1.info
            );
        }
    }
}

#[test]
fn no_equal_sign_errors() {
    let path = "./tests/fixtures/parser/ftl/errors/04-no-equal-sign.ftl";
    let source = read_file(path).expect("Failed to read");
    match parse(&source) {
        Ok(_) => panic!("Expected errors in the file"),
        Err((_, ref errors)) => {
            assert_eq!(1, errors.len());

            let error1 = &errors[0];

            assert_eq!(ErrorKind::ExpectedToken { token: '=' }, error1.kind);

            assert_eq!(
                Some(ErrorInfo {
                    slice: "key Value".to_owned(),
                    line: 0,
                    col: 4,
                    pos: 4,
                },),
                error1.info
            );
        }
    }
}

#[test]
fn wrong_char_in_id_errors() {
    let path = "./tests/fixtures/parser/ftl/errors/05-bad-char-in-keyword.ftl";
    let source = read_file(path).expect("Failed to read");
    match parse(&source) {
        Ok(_) => panic!("Expected errors in the file"),
        Err((_, ref errors)) => {
            assert_eq!(1, errors.len());

            let error1 = &errors[0];

            assert_eq!(
                ErrorKind::ExpectedCharRange {
                    range: "'a'...'z' | 'A'...'Z'".to_owned(),
                },
                error1.kind
            );

            assert_eq!(
                Some(ErrorInfo {
                    slice: "key = Value\n .# = Foo".to_owned(),
                    line: 0,
                    col: 2,
                    pos: 14,
                },),
                error1.info
            );
        }
    }
}

#[test]
fn missing_trait_value_errors() {
    let path = "./tests/fixtures/parser/ftl/errors/06-trait-value.ftl";
    let source = read_file(path).expect("Failed to read");
    match parse(&source) {
        Ok(_) => panic!("Expected errors in the file"),
        Err((_, ref errors)) => {
            assert_eq!(1, errors.len());

            let error1 = &errors[0];

            assert_eq!(ErrorKind::ExpectedToken { token: '\u{2424}' }, error1.kind);

            assert_eq!(
                Some(ErrorInfo {
                    slice: "key = Value\n .foo".to_owned(),
                    line: 0,
                    col: 5,
                    pos: 17,
                },),
                error1.info
            );
        }
    }
}

#[test]
fn message_missing_fields_errors() {
    let path = "./tests/fixtures/parser/ftl/errors/07-message-missing-fields.ftl";
    let source = read_file(path).expect("Failed to read");
    match parse(&source) {
        Ok(_) => panic!("Expected errors in the file"),
        Err((_, ref errors)) => {
            assert_eq!(1, errors.len());

            let error1 = &errors[0];

            assert_eq!(ErrorKind::ExpectedToken { token: '\u{2424}' }, error1.kind);

            assert_eq!(
                Some(ErrorInfo {
                    slice: "key".to_owned(),
                    line: 0,
                    col: 3,
                    pos: 3,
                },),
                error1.info
            );
        }
    }
}

#[test]
fn private_errors() {
    let path = "./tests/fixtures/parser/ftl/errors/08-private.ftl";
    let source = read_file(path).expect("Failed to read");
    match parse(&source) {
        Ok(_) => panic!("Expected errors in the file"),
        Err((_, ref errors)) => {
            assert_eq!(4, errors.len());

            let error1 = &errors[0];

            assert_eq!(
                ErrorKind::ExpectedCharRange {
                    range: "0...9".to_owned(),
                },
                error1.kind
            );

            assert_eq!(
                Some(ErrorInfo {
                    slice: "key =\n    { $foo ->\n        [one] Foo\n       *[-other] Foo 2\n    }"
                        .to_owned(),
                    line: 1,
                    col: 10,
                    pos: 48,
                },),
                error1.info
            );

            let error2 = &errors[1];

            assert_eq!(
                ErrorKind::ExpectedCharRange {
                    range: "'a'...'z' | 'A'...'Z'".to_owned(),
                },
                error2.kind
            );

            assert_eq!(
                Some(ErrorInfo {
                    slice: "key2 = { $-foo }".to_owned(),
                    line: 7,
                    col: 10,
                    pos: 10,
                },),
                error2.info
            );

            let error3 = &errors[2];

            assert_eq!(ErrorKind::TermAttributeAsSelector, error3.kind);

            assert_eq!(
                Some(ErrorInfo {
                    slice: "key3 = { -brand.gender }".to_owned(),
                    line: 9,
                    col: 23,
                    pos: 23,
                },),
                error3.info
            );

            let error4 = &errors[3];

            assert_eq!(ErrorKind::ForbiddenCallee, error4.kind);

            assert_eq!(
                Some(ErrorInfo {
                    slice: "key4 = { -brand() }".to_owned(),
                    line: 11,
                    col: 15,
                    pos: 15,
                },),
                error4.info
            );
        }
    }
}
