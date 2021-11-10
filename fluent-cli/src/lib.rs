use std::cmp;
use std::fs::File;
use std::io;
use std::io::Read;

use annotate_snippets::display_list::DisplayList;
use annotate_snippets::formatter::DisplayListFormatter;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};
use fluent_syntax::ast::Resource;
use fluent_syntax::parser::parse;
use fluent_syntax::parser::ErrorKind;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn print_entries_resource(res: &Resource<&str>) {
    println!("{:#?}", res);
}

pub fn parse_file(input: &str, silent: bool) {
    let source = read_file(&input).expect("Read file failed");

    let res = parse(source.as_str());

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
                    let end_pos = cmp::min(err.pos.end, slice.end);
                    let snippet = Snippet {
                        slices: vec![Slice {
                            source: source[slice.clone()].to_string(),
                            line_start: get_line_num(&source, err.pos.start) + 1,
                            origin: Some(input.to_string()),
                            fold: false,
                            annotations: vec![SourceAnnotation {
                                label: desc.to_string(),
                                annotation_type: AnnotationType::Error,
                                range: (err.pos.start - slice.start, end_pos - slice.start + 1),
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

fn get_error_info(kind: ErrorKind) -> (String, String) {
    (format!("E0000"), kind.to_string())
}
