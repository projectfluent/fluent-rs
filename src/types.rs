pub enum FluentType {
    FluentString(String),
    FluentNone,
    FluentNumber,
}

pub fn value_of(t: FluentType) -> String {
    match t {
        FluentType::FluentString(s) => s,
        _ => unimplemented!(),
    }
}
