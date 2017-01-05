#[derive(Debug)]
pub enum ParserError {
    Generic,
    ExpectedToken { token: char },
    ExpectedCharRange { range: String },
    ExpectedField { field: String },
    MissingField { fields: Vec<String> },
}

fn get_line_for_pos(source: &str, pos: usize) -> (usize, usize) {
    let mut lines = source.lines();
    let mut ptr = 0;
    let mut line_num = 0;

    while let Some(line) = lines.next() {
        let line_len = line.len();
        line_num += 1;

        if ptr + line_len > pos {
            return (ptr, line_num);
        }

    }
    return (0, 0);
}

pub fn get_error_slice(source: &str, start: usize) -> &str {
    let len = source.len();

    let mut start_pos = 0;
    let mut slice_len = 10;

    //println!("len: {:?}", len);
    if len <= slice_len {
        start_pos = 0;
        slice_len = len;
    } else if start + slice_len >= len {
        start_pos = len - slice_len - 1;
    } else {
        start_pos = start;
    }

    // println!("start_pos: {:?}", start_pos);
    // println!("slice_len: {:?}", slice_len);

    let mut iter = source.chars();
    if start_pos > 0 {
        iter.by_ref().nth(start_pos - 1);
    }
    let slice = iter.as_str();
    let endp = slice.char_indices().nth(slice_len).map(|(n, _)| n).unwrap_or(len);
    return &slice[..endp];
}
