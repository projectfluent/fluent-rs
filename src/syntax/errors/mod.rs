extern crate term;

pub mod display;
mod list;

pub use self::list::ItemName as items;
pub use self::list::get_item;
use std::fmt;

macro_rules! error {
    ($kind:expr) => {{
        Err(ParserError {
            info: None,
            kind: $kind
        })
    }};
}

#[derive(Debug, PartialEq)]
pub struct ErrorInfo {
    pub slice: String,
    pub line: usize,
    pub pos: usize,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Generic,
    ExpectedEntry,
    ExpectedToken { token: char },
    ExpectedCharRange { range: String },
    ExpectedField { field: String },
    MissingField { fields: Vec<String> },
    MissingDefaultVariant,
    MissingVariants,
    ForbiddenWhitespace,
    ForbiddenCallee,
    ForbiddenKey,
}

#[derive(Debug)]
pub struct ParserError {
    pub info: Option<ErrorInfo>,
    pub kind: ErrorKind,
}

fn get_error_desc(err: &ErrorKind) -> (String, String) {
    match err {
        &ErrorKind::ExpectedEntry => {
            return ("E0003".to_owned(),
                    "Expected an entry start ('a'...'Z' | '_' | '[[' | '#')".to_owned());
        }
        &ErrorKind::ExpectedCharRange { ref range } => {
            return ("E0004".to_owned(), format!("Expected a character from range ({})", range));
        }
        &ErrorKind::MissingField { ref fields } => {
            let list = fields.join(", ");
            return ("E0005".to_owned(), format!("Expected one of the fields: {}", list));
        }
        &ErrorKind::ExpectedField { ref field } => {
            return ("E0006".to_owned(), format!("Expected field: {}", field));
        }
        &ErrorKind::ExpectedToken { token } => {
            return ("E0001".to_owned(), format!("expected token `{}`", token));
        }
        &ErrorKind::ForbiddenWhitespace => {
            return ("E0007".to_owned(), "keyword cannot end with a whitespace".to_owned());
        }
        &ErrorKind::ForbiddenCallee => {
            return ("E0008".to_owned(), "a callee has to be a simple identifier".to_owned());
        }
        &ErrorKind::ForbiddenKey => {
            return ("E0009".to_owned(), "a key has to be a simple identifier".to_owned());
        }
        &ErrorKind::MissingDefaultVariant => {
            return ("E0010".to_owned(),
                    "Expected one of the variants to be marked as default (*).".to_owned());
        }
        &ErrorKind::MissingVariants => {
            return ("E0010".to_owned(), "Expected at least one variant after \"->\".".to_owned());
        }
        &ErrorKind::Generic => {
            return ("E0002".to_owned(), "generic error".to_owned());
        }
    }
}

// fn get_error_hint(err: ErrorKind) -> str {
//     "Did you mean to put \"=\" after the ID?".to_owned()
// }

fn draw_line(line_num: usize, max_dig_space: usize) -> String {

    let dig_diff = if line_num == 0 {
        max_dig_space
    } else {
        let dig_space = line_num.to_string().len();
        max_dig_space - dig_space
    };

    let mut ln = (0..dig_diff).map(|_| " ").collect::<String>();
    if line_num != 0 {
        ln.push_str(&line_num.to_string());
    }
    return ln;
}

fn draw_error_line(max_dig_space: usize, col: usize) -> (String, String) {
    let ln = (0..max_dig_space).map(|_| " ").collect::<String>();
    let mut ln2 = (0..col).map(|_| " ").collect::<String>();
    ln2.push_str("^");
    return (ln, ln2);
}

impl fmt::Display for ParserError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{:#?}\n\n", self)?;
        //
        let mut t = term::stdout().unwrap();

        let (error_name, error_desc) = get_error_desc(&self.kind);
        t.fg(term::color::BRIGHT_RED).unwrap();
        t.attr(term::Attr::Bold).unwrap();
        write!(t, "error[{}]", error_name).unwrap();
        t.fg(term::color::WHITE).unwrap();
        write!(t, ": {}\n", error_desc).unwrap();
        t.reset().unwrap();

        if let Some(ref info) = self.info {
            let lines = info.slice.lines();
            let mut i = info.line + 1;

            let (error_line, error_col) = get_line_pos(&info.slice, info.pos);

            let max_dig_space = (i + lines.count()).to_string().len();


            let ln = draw_line(0, max_dig_space);
            t.fg(term::color::BRIGHT_BLUE).unwrap();
            t.attr(term::Attr::Bold).unwrap();
            write!(t, "{} | ", ln).unwrap();
            t.reset().unwrap();
            write!(t, "\n").unwrap();

            let lines = info.slice.lines();
            let mut j = 0;

            for line in lines {
                let ln = draw_line(i, max_dig_space);
                t.fg(term::color::BRIGHT_BLUE).unwrap();
                t.attr(term::Attr::Bold).unwrap();
                write!(t, "{} | ", ln).unwrap();
                t.reset().unwrap();
                write!(t, "{}\n", line).unwrap();

                if j == error_line {
                    let (ln, ln2) = draw_error_line(max_dig_space, error_col);
                    t.fg(term::color::BRIGHT_BLUE).unwrap();
                    t.attr(term::Attr::Bold).unwrap();
                    write!(t, "{} | ", ln).unwrap();
                    t.reset().unwrap();
                    t.fg(term::color::BRIGHT_RED).unwrap();
                    t.attr(term::Attr::Bold).unwrap();
                    write!(t, "{}\n", ln2).unwrap();
                    t.reset().unwrap();
                }
                j += 1;
                i += 1;
            }

            if j == 0 {
                let ln = draw_line(i, max_dig_space);
                t.fg(term::color::BRIGHT_BLUE).unwrap();
                t.attr(term::Attr::Bold).unwrap();
                write!(t, "{} | ", ln).unwrap();
                t.reset().unwrap();
                write!(t, "\n").unwrap();
            }
        }
        Ok(())
    }
}

pub fn get_line_pos(source: &str, pos: usize) -> (usize, usize) {
    let mut ptr = 0;
    let mut i = 0;

    let lines = source.lines();

    for line in lines {
        let len = line.chars().count();
        if ptr + len + 1 > pos {
            break;
        }

        ptr += len + 1;
        i += 1;
    }

    return (i, pos - ptr);
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

    return i;
}

pub fn get_error_lines(source: &str, start: usize, end: usize) -> String {

    let l = if start < end { end - start } else { 1 };

    let lines = source.lines().skip(start).take(l);

    let mut s = String::new();

    for line in lines {
        s.push_str(line);
        s.push('\n');
    }

    return String::from(s.trim_right());
}

pub fn get_error_slice(source: &str, start: usize, end: usize) -> &str {
    let len = source.chars().count();

    let start_pos;
    let mut slice_len = end - start;

    if len <= slice_len {
        start_pos = 0;
        slice_len = len;
    } else if start + slice_len >= len {
        start_pos = len - slice_len - 1;
    } else {
        start_pos = start;
    }

    let mut iter = source.chars();
    if start_pos > 0 {
        iter.by_ref().nth(start_pos - 1);
    }
    let slice = iter.as_str();
    let endp = slice.char_indices().nth(slice_len).map(|(n, _)| n).unwrap_or(len);
    return &slice[..endp];
}

pub fn get_error_info(source: &str,
                      pos: usize,
                      entry_start: usize,
                      next_entry_start: usize)
                      -> Option<ErrorInfo> {
    let first_line_num = get_line_num(source, entry_start);
    let next_entry_line = get_line_num(source, next_entry_start);

    let slice = get_error_lines(source, first_line_num, next_entry_line);

    Some(ErrorInfo {
        slice: slice,
        line: first_line_num,
        pos: pos - entry_start,
    })
}
