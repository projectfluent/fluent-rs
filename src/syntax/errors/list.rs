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
    ExpectedToken {
        token: char,
    },
    ExpectedCharRange {
        range: String,
    },
    ExpectedField {
        field: String,
    },
    MissingField {
        entry_id: String,
        fields: Vec<&'static str>,
    },
    MissingDefaultVariant,
    MissingVariants,
    ForbiddenWhitespace,
    ForbiddenCallee,
    ForbiddenKey,
    ForbiddenPrivateAttributeExpression,
    ForbiddenPublicAttributeExpression,
    ForbiddenVariantExpression,
}

pub fn get_error_desc(err: &ErrorKind) -> (&'static str, String, &'static str) {
    match *err {
        ErrorKind::Generic => ("E0001", "generic error".to_owned(), ""),
        ErrorKind::ExpectedEntry => (
            "E0002",
            "Expected an entry start".to_owned(),
            "Expected one of ('a'...'Z' | '_' | '[[' | '#') here",
        ),
        ErrorKind::ExpectedToken { token } => ("E0003", format!("expected token `{}`", token), ""),
        ErrorKind::ExpectedCharRange { ref range } => (
            "E0004",
            format!("Expected a character from range ({})", range),
            "",
        ),
        ErrorKind::MissingField {
            ref entry_id,
            ref fields,
        } => {
            let list = fields.join(", ");
            (
                "E0005",
                format!(
                    "Expected entry `{}` to have one of the fields: {}",
                    entry_id, list
                ),
                "",
            )
        }
        ErrorKind::ExpectedField { ref field } => {
            ("E0006", format!("Expected field: {}", field), "")
        }
        ErrorKind::ForbiddenWhitespace => (
            "E0007",
            "Keyword cannot end with a whitespace".to_owned(),
            "",
        ),
        ErrorKind::ForbiddenCallee => (
            "E0008",
            "The callee has to be a simple, upper-case, identifier".to_owned(),
            "",
        ),
        ErrorKind::ForbiddenKey => (
            "E0009",
            "The key has to be a simple identifier".to_owned(),
            "",
        ),
        ErrorKind::MissingDefaultVariant => (
            "E0010",
            "Expected one of the variants to be marked as default (*).".to_owned(),
            "",
        ),
        ErrorKind::MissingVariants => (
            "E0011",
            "Expected at least one variant after \"->\".".to_owned(),
            "",
        ),
        ErrorKind::ForbiddenPrivateAttributeExpression => (
            "E0012",
            "Attributes of private messages cannot be used as placeables.".to_owned(),
            "",
        ),
        ErrorKind::ForbiddenPublicAttributeExpression => (
            "E0013",
            "Attributes of public messages cannot be used as selectors.".to_owned(),
            "",
        ),
        ErrorKind::ForbiddenVariantExpression => (
            "E0014",
            "Variants cannot be used as selectors.".to_owned(),
            "",
        ),
    }
}
