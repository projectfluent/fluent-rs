use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct ParserError {
    pub pos: (usize, usize),
    pub slice: Option<(usize, usize)>,
    pub kind: ErrorKind,
}

impl std::error::Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

macro_rules! error {
    ($kind:expr, $start:expr) => {{
        Err(ParserError {
            pos: ($start, $start + 1),
            slice: None,
            kind: $kind,
        })
    }};
    ($kind:expr, $start:expr, $end:expr) => {{
        Err(ParserError {
            pos: ($start, $end),
            slice: None,
            kind: $kind,
        })
    }};
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Generic,
    ExpectedEntry,
    ExpectedToken(char),
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
    TermReferenceAsSelector,
    MessageAttributeAsSelector,
    TermAttributeAsPlaceable,
    UnterminatedStringExpression,
    PositionalArgumentFollowsNamed,
    DuplicatedNamedArgument(String),
    ForbiddenVariantAccessor,
    UnknownEscapeSequence(String),
    InvalidUnicodeEscapeSequence(String),
    UnbalancedClosingBrace,
    ExpectedInlineExpression,
    ExpectedSimpleExpressionAsSelector,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Generic => write!(f, "An error occurred"),
            ErrorKind::ExpectedEntry => write!(f, "Expected an entry"),
            ErrorKind::ExpectedToken(letter) => {
                write!(f, "Expected a token starting with \"{}\"", letter)
            }
            ErrorKind::ExpectedCharRange { range } => write!(f, "Expected one of \"{}\"", range),
            ErrorKind::ExpectedMessageField { entry_id } => {
                write!(f, "Expected a message field for \"{}\"", entry_id)
            }
            ErrorKind::ExpectedTermField { entry_id } => {
                write!(f, "Expected a term field for \"{}\"", entry_id)
            }
            ErrorKind::ForbiddenWhitespace => write!(f, "Whitespace is not allowed here"),
            ErrorKind::ForbiddenCallee => write!(f, "Callee is not allowed here"),
            ErrorKind::ForbiddenKey => write!(f, "Key is not allowed here"),
            ErrorKind::MissingDefaultVariant => {
                write!(f, "The select expression must have a default variant")
            }
            ErrorKind::MissingVariants => {
                write!(f, "The select expression must have one or more variants")
            }
            ErrorKind::MissingValue => write!(f, "Expected a value"),
            ErrorKind::MissingVariantKey => write!(f, "Expected a variant key"),
            ErrorKind::MissingLiteral => write!(f, "Expected a literal"),
            ErrorKind::MultipleDefaultVariants => {
                write!(f, "A select expression can only have one default variant",)
            }
            ErrorKind::MessageReferenceAsSelector => {
                write!(f, "Message references can't be used as a selector")
            }
            ErrorKind::TermReferenceAsSelector => {
                write!(f, "Term references can't be used as a selector")
            }
            ErrorKind::MessageAttributeAsSelector => {
                write!(f, "Message attributes can't be used as a selector")
            }
            ErrorKind::TermAttributeAsPlaceable => {
                write!(f, "Term attributes can't be used as a placeable")
            }
            ErrorKind::UnterminatedStringExpression => write!(f, "Unterminated string expression"),
            ErrorKind::PositionalArgumentFollowsNamed => {
                write!(f, "Positional arguments must come before named arguments",)
            }
            ErrorKind::DuplicatedNamedArgument(name) => {
                write!(f, "The \"{}\" argument appears twice", name)
            }
            ErrorKind::ForbiddenVariantAccessor => write!(f, "Forbidden variant accessor"),
            ErrorKind::UnknownEscapeSequence(seq) => {
                write!(f, "Unknown escape sequence, \"{}\"", seq)
            }
            ErrorKind::InvalidUnicodeEscapeSequence(seq) => {
                write!(f, "Invalid unicode escape sequence, \"{}\"", seq)
            }
            ErrorKind::UnbalancedClosingBrace => write!(f, "Unbalanced closing brace"),
            ErrorKind::ExpectedInlineExpression => write!(f, "Expected an inline expression"),
            ErrorKind::ExpectedSimpleExpressionAsSelector => {
                write!(f, "Expected a simple expression as selector")
            }
        }
    }
}
