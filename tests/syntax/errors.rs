extern crate fluent;

use std::io::prelude::*;
use std::fs::File;
use std::io;

use self::fluent::syntax::parse;
use self::fluent::syntax::errors::ErrorKind;
use self::fluent::syntax::errors::ErrorInfo;

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

            let ref error1 = errors[0];

            assert_eq!(ErrorKind::ExpectedEntry, error1.kind);

            assert_eq!(Some(ErrorInfo {
                           slice: "".to_owned(),
                           line: 0,
                           pos: 0,
                       }),
                       error1.info);

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

            let ref error1 = errors[0];

            assert_eq!(ErrorKind::ExpectedEntry, error1.kind);

            assert_eq!(Some(ErrorInfo {
                           slice: "2".to_owned(),
                           line: 0,
                           pos: 0,
                       }),
                       error1.info);

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

            let ref error1 = errors[0];

            assert_eq!(ErrorKind::ExpectedToken { token: '=' }, error1.kind);

            assert_eq!(Some(ErrorInfo {
                           slice: "key".to_owned(),
                           line: 0,
                           pos: 3,
                       }),
                       error1.info);

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

            let ref error1 = errors[0];

            assert_eq!(ErrorKind::ExpectedToken { token: '=' }, error1.kind);

            assert_eq!(Some(ErrorInfo {
                           slice: "key Value".to_owned(),
                           line: 0,
                           pos: 4,
                       }),
                       error1.info);

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

            let ref error1 = errors[0];

            assert_eq!(ErrorKind::ExpectedCharRange {
                           range: "'a'...'z' | 'A'...'Z' | '_'".to_owned(),
                       },
                       error1.kind);

            assert_eq!(Some(ErrorInfo {
                           slice: "key = Value\n [#] Foo".to_owned(),
                           line: 0,
                           pos: 14,
                       }),
                       error1.info);

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

            let ref error1 = errors[0];

            assert_eq!(ErrorKind::ExpectedField { field: "Pattern".to_owned() },
                       error1.kind);

            assert_eq!(Some(ErrorInfo {
                           slice: "key = Value\n [foo]".to_owned(),
                           line: 0,
                           pos: 18,
                       }),
                       error1.info);

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

            let ref error1 = errors[0];

            assert_eq!(ErrorKind::MissingField {
                           fields: vec!["Value".to_owned(), "Traits".to_owned()],
                       },
                       error1.kind);

            assert_eq!(Some(ErrorInfo {
                           slice: "key =".to_owned(),
                           line: 0,
                           pos: 5,
                       }),
                       error1.info);

        }
    }
}
