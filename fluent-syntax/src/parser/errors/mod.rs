#[derive(Debug, PartialEq)]
pub struct ParserError {
    pub pos: usize,
    pub slice: Option<(usize, usize)>,
    pub kind: ErrorKind,
}

macro_rules! error {
    ($ps:ident, $kind:expr) => {{
        Err(ParserError {
            pos: $ps.ptr,
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
    VariantAsSelector,
    MessageAttributeAsSelector,
    TermAttributeAsPlaceable,
    UnterminatedStringExpression,
    PositionalArgumentFollowsNamed,
    DuplicatedNamedArgument(String),
    VariantListInExpression,
    ForbiddenVariantAccessor,
    UnknownEscapeSequence(String),
    InvalidUnicodeEscapeSequence(String),
}
