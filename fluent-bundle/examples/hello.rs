use fluent_bundle::bundle::FluentBundle;

fn main() {
    let mut bundle = FluentBundle::new(&["en-US"]);
    bundle.add_messages("hello-world = Hello, world!").unwrap();
    let (value, _) = bundle.format("hello-world", None).unwrap();
    assert_eq!(&value, "Hello, world!");
}
