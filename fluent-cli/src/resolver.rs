use std::io;

use fluent_bundle::{FluentBundle, FluentResource};

use serde::Serialize;

#[derive(Default, Serialize)]
struct Output {
    value: String,
    errors: Vec<bool>
}

fn resolve(s: String) -> String {
    let mut bundle = FluentBundle::default();
    let res = FluentResource::try_new(s).expect("Failed to parse input");
    bundle.add_resource(res).expect("Failed to add resource");

    let msg = bundle
        .get_message("test")
        .expect("Failed to retrieve a test message");

    let mut errors = vec![];
    let value = bundle.format_pattern(msg.value.expect("Message has no value"), None, &mut errors).into();
    let output = Output {
        value,
        ..Default::default()
    };
    return serde_json::to_string(&output).expect("Serializing JSON failed.");
}

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let result = resolve(input);
            println!("{}", result);
        }
        Err(error) => println!("error: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = resolve("test = Value".into());
        assert_eq!(x, r#"{"value":"Value","errors":[]}"#);
    }
}
