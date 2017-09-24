pub mod display;
mod list;

pub use self::list::ParserError;
pub use self::list::ErrorKind;
pub use self::list::ErrorInfo;
pub use self::list::get_error_desc;

macro_rules! error {
    ($kind:expr) => {{
        Err(ParserError {
            info: None,
            kind: $kind
        })
    }};
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

fn get_col_num(source: &str, pos: usize) -> usize {
    let mut ptr = 0;

    let lines = source.lines();

    for line in lines {
        let lnlen = line.chars().count();

        if ptr + lnlen + 1 > pos {
            return pos - ptr;
        }

        ptr += lnlen + 1;
    }

    0
}

pub fn get_error_lines(source: &str, start: usize, end: usize) -> String {

    let l = if start < end { end - start } else { 1 };

    let lines = source.lines().skip(start).take(l);

    let mut s = String::new();

    for line in lines {
        s.push_str(line);
        s.push('\n');
    }

    String::from(s.trim_right())
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
    let endp = slice
        .char_indices()
        .nth(slice_len)
        .map(|(n, _)| n)
        .unwrap_or(len);
    &slice[..endp]
}

pub fn get_error_info(
    source: &str,
    pos: usize,
    entry_start: usize,
    next_entry_start: usize,
) -> Option<ErrorInfo> {
    let first_line_num = get_line_num(source, entry_start);
    let next_entry_line = get_line_num(source, next_entry_start);

    let slice = get_error_lines(source, first_line_num, next_entry_line);

    Some(ErrorInfo {
        slice: slice,
        line: first_line_num,
        col: get_col_num(source, pos),
        pos: pos - entry_start,
    })
}
