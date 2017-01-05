#[derive(Debug)]
pub enum ParserError {
    Generic,
    ExpectedToken { token: char },
    ExpectedCharRange { range: String },
    ExpectedField { field: String },
    MissingField { fields: Vec<String> },
}
