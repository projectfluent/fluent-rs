#[derive(Clone, Debug, PartialEq)]
pub enum FluentValue {
    String(String),
    Number(f32),
}

impl FluentValue {
    pub fn format(&self) -> String {
        match *self {
            FluentValue::String(ref s) => s.clone(),
            FluentValue::Number(ref n) => format!("{}", n),
        }
    }
}

impl From<String> for FluentValue {
    fn from(s: String) -> Self {
        FluentValue::String(s)
    }
}

impl From<&'static str> for FluentValue {
    fn from(s: &'static str) -> Self {
        FluentValue::String(String::from(s))
    }
}

impl From<f32> for FluentValue {
    fn from(n: f32) -> Self {
        FluentValue::Number(n)
    }
}

impl From<i8> for FluentValue {
    fn from(n: i8) -> Self {
        FluentValue::Number(n as f32)
    }
}
