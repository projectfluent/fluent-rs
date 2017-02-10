#[derive(Debug)]
pub struct ParserError {
    pub info: Option<ErrorInfo>,
    pub kind: ErrorKind,
}

#[derive(Debug, PartialEq)]
pub struct ErrorInfo {
    pub slice: String,
    pub line: usize,
    pub col: usize,
    pub pos: usize,
}


#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Generic,
    ExpectedEntry,
    ExpectedToken { token: char },
    ExpectedCharRange { range: String },
    ExpectedField { field: String },
    MissingField { entry_id: String, fields: Vec<&'static str> },
    MissingDefaultVariant,
    MissingVariants,
    ForbiddenWhitespace,
    ForbiddenCallee,
    ForbiddenKey,
}

pub fn get_error_desc(err: &ErrorKind) -> (&'static str, String) {
    match err {
        &ErrorKind::Generic => {
            return ("E0001", "generic error".to_owned());
        }
        &ErrorKind::ExpectedEntry => {
            return ("E0002",
                    "Expected an entry start ('a'...'Z' | '_' | '[[' | '#')".to_owned());
        }
        &ErrorKind::ExpectedToken { token } => {
            return ("E0003", format!("expected token `{}`", token));
        }
        &ErrorKind::ExpectedCharRange { ref range } => {
            return ("E0004", format!("Expected a character from range ({})", range));
        }
        &ErrorKind::MissingField { ref entry_id, ref fields } => {
            let list = fields.join(", ");
            return ("E0005", format!("Expected entry `{}` to have one of the fields: {}", entry_id, list));
        }
        &ErrorKind::ExpectedField { ref field } => {
            return ("E0006", format!("Expected field: {}", field));
        }
        &ErrorKind::ForbiddenWhitespace => {
            return ("E0007", "keyword cannot end with a whitespace".to_owned());
        }
        &ErrorKind::ForbiddenCallee => {
            return ("E0008", "a callee has to be a simple identifier".to_owned());
        }
        &ErrorKind::ForbiddenKey => {
            return ("E0009", "a key has to be a simple identifier".to_owned());
        }
        &ErrorKind::MissingDefaultVariant => {
            return ("E0010",
                    "Expected one of the variants to be marked as default (*).".to_owned());
        }
        &ErrorKind::MissingVariants => {
            return ("E0010", "Expected at least one variant after \"->\".".to_owned());
        }
    }
}
