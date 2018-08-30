#[derive(Debug, PartialEq)]
pub struct ParserError {
    pub info: Option<ErrorInfo>,
    pub kind: ErrorKind,
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
    ExpectedMessageField { entry_id: String },
    ExpectedTermField { entry_id: String },
    ForbiddenWhitespace,
    ForbiddenCallee,
    ForbiddenKey,
    MissingDefaultVariant,
    MissingVariants,
    MissingValue,
    MissingVariantKey,
    MissingLiteral,
    MultipleDefaultVariants,
    MessageReferenceAsSelector,
    VariantAsSelector,
    MessageAttributeAsSelector,
    TermAttributeAsSelector,
    UnterminatedStringExpression,
}

pub fn get_error_desc(err: &ErrorKind) -> (&'static str, String, &'static str) {
    match err {
        ErrorKind::Generic => ("E0001", "generic error".to_owned(), ""),
        ErrorKind::ExpectedEntry => (
            "E0002",
            "Expected an entry start".to_owned(),
            "Expected one of ('a'...'Z' | '_' | #') here",
        ),
        ErrorKind::ExpectedToken { token } => ("E0003", format!("expected token `{}`", token), ""),
        ErrorKind::ExpectedCharRange { range } => (
            "E0004",
            format!("Expected a character from range ({})", range),
            "",
        ),
        ErrorKind::ExpectedMessageField { entry_id } => (
            "E0005",
            format!(
                "Expected message `{}` to have a value or attributes",
                entry_id
            ),
            "",
        ),
        ErrorKind::ExpectedTermField { entry_id } => (
            "E0006",
            format!("Expected term `{}` to have a value", entry_id),
            "",
        ),
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
        ErrorKind::MissingValue => ("E0012", "Expected value".to_owned(), ""),
        ErrorKind::MissingVariantKey => ("E0013", "Expected variant key".to_owned(), ""),
        ErrorKind::MissingLiteral => ("E0014", "Expected literal".to_owned(), ""),
        ErrorKind::MultipleDefaultVariants => (
            "E0015",
            "Only one variant can be marked as default (*)".to_owned(),
            "",
        ),
        ErrorKind::MessageReferenceAsSelector => (
            "E0016",
            "Message references cannot be used as selectors".to_owned(),
            "",
        ),
        ErrorKind::VariantAsSelector => (
            "E0017",
            "Variants cannot be used as selectors".to_owned(),
            "",
        ),
        ErrorKind::MessageAttributeAsSelector => (
            "E0018",
            "Attributes of messages cannot be used as selectors.".to_owned(),
            "",
        ),
        ErrorKind::TermAttributeAsSelector => (
            "E0019",
            "Attributes of terms cannot be used as selectors.".to_owned(),
            "",
        ),
        ErrorKind::UnterminatedStringExpression => {
            ("E0020", "Underminated string expression".to_owned(), "")
        }
    }
}
