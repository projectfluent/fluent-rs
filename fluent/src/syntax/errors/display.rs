extern crate annotate_snippets;

use super::list::get_error_desc;
use super::ParserError;

use self::annotate_snippets::display_list::DisplayList;
use self::annotate_snippets::formatter::DisplayListFormatter;
use self::annotate_snippets::snippet;

pub fn annotate_error(err: &ParserError, file_name: &Option<String>, color: bool) -> String {
    let desc = get_error_desc(&err.kind);

    let (source, line_start, pos) = if let Some(ref info) = err.info {
        (info.slice.clone(), info.line, info.pos)
    } else {
        panic!()
    };

    let snippet = snippet::Snippet {
        slices: vec![snippet::Slice {
            source,
            line_start,
            origin: file_name.clone(),
            fold: false,
            annotations: vec![snippet::SourceAnnotation {
                label: desc.2.to_string(),
                annotation_type: snippet::AnnotationType::Error,
                range: (pos, pos + 1),
            }],
        }],
        title: Some(snippet::Annotation {
            label: Some(desc.1),
            id: Some(desc.0.to_string()),
            annotation_type: snippet::AnnotationType::Error,
        }),
        footer: vec![],
    };
    let dl = DisplayList::from(snippet);
    let dlf = DisplayListFormatter::new(color);
    dlf.format(&dl)
}
