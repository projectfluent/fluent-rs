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
