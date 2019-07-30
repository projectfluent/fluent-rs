use std::cmp;
use std::fs::File;
use std::io;
use std::io::Read;

use annotate_snippets::display_list::DisplayList;
use annotate_snippets::formatter::DisplayListFormatter;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};
use fluent_syntax::ast::Resource;
use fluent_syntax::parser::errors::ErrorKind;
use fluent_syntax::parser::parse;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn print_entries_resource(res: &Resource) {
    println!("{:#?}", res);
}

pub fn parse_file(input: &str, silent: bool) {
    let source = read_file(&input).expect("Read file failed");

    let res = parse(&source);

    match res {
        Ok(res) => print_entries_resource(&res),
        Err((res, errors)) => {
            print_entries_resource(&res);

            if silent {
                return;
            };

            println!("==============================\n");
            if errors.len() == 1 {
                println!("Parser encountered one error:");
            } else {
                println!("Parser encountered {} errors:", errors.len());
            }
            println!("-----------------------------");
            for err in errors {
                println!("{:#?}", err);
                if let Some(slice) = err.slice {
                    let (id, desc) = get_error_info(err.kind);
                    let end_pos = cmp::min(err.pos.1, slice.1);
                    let snippet = Snippet {
                        slices: vec![Slice {
                            source: source[slice.0..slice.1].to_string(),
                            line_start: get_line_num(&source, err.pos.0) + 1,
                            origin: Some(input.to_string()),
                            fold: false,
                            annotations: vec![SourceAnnotation {
                                label: desc.to_string(),
                                annotation_type: AnnotationType::Error,
                                range: (err.pos.0 - slice.0, end_pos - slice.0 + 1),
                            }],
                        }],
                        title: Some(Annotation {
                            label: Some(desc.to_string()),
                            id: Some(id.to_string()),
                            annotation_type: AnnotationType::Error,
                        }),
                        footer: vec![],
                    };
                    let dl = DisplayList::from(snippet);
                    let dlf = DisplayListFormatter::new(true, false);
                    println!("{}", dlf.format(&dl));
                    println!("-----------------------------");
                }
            }
        }
    };
}

fn get_line_num(source: &str, pos: usize) -> usize {
    let mut ptr = 0;
    let mut i = 0;

    let lines = source.lines();

    for line in lines {
        let lnlen = line.chars().count();
        ptr += lnlen + 1;

        if ptr > pos {
            break;
        }
        i += 1;
    }

    i
}

fn get_error_info(kind: ErrorKind) -> (&'static str, String) {
    match kind {
        ErrorKind::Generic => ("E0001", "Generic error".to_string()),
        ErrorKind::ExpectedEntry => ("E0002", "Expected an entry start".to_string()),
        ErrorKind::ExpectedToken(ch) => ("E0003", format!("Expected token: \"{}\"", ch)),
        ErrorKind::ExpectedCharRange { range } => (
            "E0004",
            format!("Expected a character from range: \"{}\"", range),
        ),
        ErrorKind::ExpectedMessageField { entry_id } => (
            "E0005",
            format!(
                "Expected message \"{}\" to have a value or attributes",
                entry_id
            ),
        ),
        ErrorKind::ExpectedTermField { entry_id } => (
            "E0006",
            format!("Expected term \"{}\" to have a value", entry_id),
        ),
        ErrorKind::ForbiddenWhitespace => {
            ("E0007", "Keyword cannot end with a whitespace".to_string())
        }
        ErrorKind::ForbiddenCallee => (
            "E0008",
            "The callee has to be a simple, upper-case identifier".to_string(),
        ),
        ErrorKind::ForbiddenKey => ("E0009", "The key has to be a simple identifier".to_string()),
        ErrorKind::MissingDefaultVariant => (
            "E0010",
            "Expected one of the variants to be marked as default *)".to_string(),
        ),
        ErrorKind::MissingValue => ("E0012", "Expected value.".to_string()),
        ErrorKind::TermAttributeAsPlaceable => (
            "E0019",
            "Attributes of terms cannot be used as placeables".to_string(),
        ),
        _ => ("E0000", "Unknown Error.".to_string()),
    }
}
