extern crate ansi_term;

use std::cmp;
use self::ansi_term::Colour::{Fixed, White};

use super::list;

pub fn annotate_slice(slice: String, file_name: Option<String>, item: &list::Item, start_line: usize) -> String
{
    let mut result = String::new();

    let lines_num = cmp::max(slice.lines().count(), 1);
    let max_ln_width = get_ln_width(start_line + lines_num);

    result += &format_title_line(item);
    if let Some(name) = file_name {
        result += &format_pos_line(name, start_line, 0, max_ln_width);
    }
    result += &format_slice(slice, lines_num, max_ln_width, start_line);

    return result;
}

fn format_title_line(item: &list::Item) -> String
{
    let kind = match item.kind {
        list::ItemKind::Error => "error",
        list::ItemKind::Warning => "warning"
    };

    let color = match item.kind {
        list::ItemKind::Error => Fixed(9),
        list::ItemKind::Warning => Fixed(11),
    };

    let id = item.num.to_string();

    let title = format!(": {}", item.title);

    let head = format!("{}[{}]", kind, id);

    return format!("{}{}\n",
        color.bold().paint(head),
        White.bold().paint(title)
    );
}

fn format_slice(slice: String, lines_num: usize, max_ln_width: usize, start_line: usize) -> String
{
    let mut result = String::new();

    let mut line_num = 0;

    let mut lines = slice.lines();

    for _ in 0..lines_num {
        let line = lines.next().unwrap_or("");

        let ln = format_ln(start_line + line_num + 1, max_ln_width); 
        result += &format!("{} {}\n",
            Fixed(12).bold().paint(format!("{} |", ln)),
            line);

        line_num += 1
    }

    return result;
}

fn get_ln_width(lines: usize) -> usize
{
    return lines.to_string().len();
}

fn format_ln(line_num: usize, max_ln_width: usize) -> String
{
    let ln_width = get_ln_width(line_num); 

    let diff = max_ln_width - ln_width;

    return " ".repeat(diff) + &line_num.to_string();
}

fn format_pos_line(file_name: String, line: usize, col: usize, max_ln_width: usize) -> String
{
    return format!("{}{} {}:{}:{}\n",
        " ".repeat(max_ln_width),
        Fixed(12).bold().paint("-->"),
        file_name,
        line + 1, col);
}
