#[derive(Debug)]
pub enum ParserError {
    Generic,
    ExpectedToken { token: char },
}
