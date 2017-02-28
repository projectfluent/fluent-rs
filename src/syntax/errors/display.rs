extern crate ansi_term;

use std::cmp;
use self::ansi_term::Colour::{Fixed, White};
use super::list::ErrorKind;
use super::list::ErrorInfo;
use super::list::get_error_desc;

pub enum Item {
    Error(ErrorKind),
    Warning,
}

pub struct Label {
    pub start_pos: usize,
    pub end_pos: usize,
    pub kind: LabelKind,
    pub text: &'static str,
}

pub enum LabelKind {
    Primary,
    Secondary,
}

pub fn annotate_slice(info: ErrorInfo, file_name: Option<String>, item: Item) -> String {
    let mut result = String::new();

    let desc = match item {
        Item::Error(ref kind) => Some(get_error_desc(kind)),
        Item::Warning => None,
    };

    let (id, title, text) = desc.unwrap_or(("", "".to_owned(), ""));

    let labels = [Label {
                      start_pos: info.pos,
                      end_pos: info.pos + 1,
                      kind: LabelKind::Primary,
                      text: text,
                  }];

    let lines_num = cmp::max(info.slice.lines().count(), 1);
    let max_ln_width = get_ln_width(info.line + lines_num);


    result += &format_title_line(&item, id, title);
    if let Some(name) = file_name {
        result += &format_pos_line(name, info.line, info.col, max_ln_width);
    }
    result += &format_slice(info.slice,
                            lines_num,
                            max_ln_width,
                            info.line,
                            &item,
                            &labels);

    return result;
}

fn format_title_line(item: &Item, id: &str, title: String) -> String {
    let kind = match item {
        &Item::Error(_) => "error",
        &Item::Warning => "warning",
    };

    let color = match item {
        &Item::Error(_) => Fixed(9),
        &Item::Warning => Fixed(11),
    };

    let title = format!(": {}", title);

    let head = format!("{}[{}]", kind, id);

    return format!("{}{}\n",
                   color.bold().paint(head),
                   White.bold().paint(title));
}

fn format_slice(slice: String,
                lines_num: usize,
                max_ln_width: usize,
                start_line: usize,
                item: &Item,
                labels: &[Label])
                -> String {
    let mut result = String::new();

    let mut line_num = 0;
    let mut pos = 0;

    let mut lines = slice.lines();

    let empty_ln = " ".repeat(max_ln_width);
    result += &Fixed(12).bold().paint(format!("{} |\n", empty_ln)).to_string();

    for i in 0..lines_num {
        let line = lines.next().unwrap_or("");

        let ln = format_ln(start_line + line_num + 1, max_ln_width);
        result += &format!("{} {}\n", Fixed(12).bold().paint(format!("{} |", ln)), line);

        let prev_line_start = pos;
        pos += line.chars().count() + 1;
        let prev_line_end = pos;

        if let Some(label_line) = format_labels(prev_line_start,
                                                prev_line_end,
                                                max_ln_width,
                                                item,
                                                labels) {
            result += &label_line;
        } else if i == lines_num - 1 {
            result += &Fixed(12).bold().paint(format!("{} |\n", empty_ln)).to_string();
        }

        line_num += 1
    }


    return result;
}

fn format_labels(start_pos: usize,
                 end_pos: usize,
                 max_ln_width: usize,
                 item: &Item,
                 labels: &[Label])
                 -> Option<String> {
    let mut result = String::new();

    for label in labels {
        if label.start_pos >= start_pos && label.start_pos <= end_pos {
            let color = match label.kind {
                LabelKind::Primary => {
                    match item {
                        &Item::Error(_) => Fixed(9).bold(),
                        &Item::Warning => Fixed(11).bold(),
                    }
                }
                LabelKind::Secondary => Fixed(12).bold(),
            };

            let line_length = end_pos - start_pos;
            let pad_pos = label.start_pos - start_pos;

            let pad = " ".repeat(pad_pos);

            let mark_length = label.end_pos - label.start_pos;

            let mark = if mark_length + pad_pos > line_length {
                "^".repeat(line_length - pad_pos) + "..."
            } else {
                "^".repeat(mark_length)
            };
            result += &format!("{} {}\n",
                               Fixed(12).bold().paint(format!("{} |", " ".repeat(max_ln_width))),
                               format!("{}{} {}", pad, color.paint(mark), color.paint(label.text)));
            return Some(result);
        }
    }

    return None;
}

fn get_ln_width(lines: usize) -> usize {
    return lines.to_string().len();
}

fn format_ln(line_num: usize, max_ln_width: usize) -> String {
    let ln_width = get_ln_width(line_num);

    let diff = max_ln_width - ln_width;

    return " ".repeat(diff) + &line_num.to_string();
}

fn format_pos_line(file_name: String, line: usize, col: usize, max_ln_width: usize) -> String {
    return format!("{}{} {}:{}:{}\n",
                   " ".repeat(max_ln_width),
                   Fixed(12).bold().paint("-->"),
                   file_name,
                   line + 1,
                   col);
}
