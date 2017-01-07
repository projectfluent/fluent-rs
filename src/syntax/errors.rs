#[derive(Debug)]
pub struct ErrorInfo {
    pub slice: String,
    pub pos: usize,
}

#[derive(Debug)]
pub enum ErrorKind {
    Generic,
    ExpectedToken { token: char },
    ExpectedCharRange { range: String },
    ExpectedField { field: String },
    MissingField { fields: Vec<String> },
}

#[derive(Debug)]
pub struct ParserError {
    pub info: Option<ErrorInfo>,
    pub kind: ErrorKind,
}

pub fn get_error_slice(source: &str, start: usize, end: usize) -> &str {
    let len = source.len();

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
