#[derive(Debug, Fail)]
pub enum FluentError {
    #[fail(display = "attempted to override an existing {}: {}", kind, id)]
    Overriding { kind: &'static str, id: String },
}
