use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::resource::FluentResource;

fn main() {
    let res = FluentResource::try_new("hello-world = Hello, world!".to_owned()).unwrap();
    let mut bundle = FluentBundle::new(&["en-US"]);
    bundle.add_resource(&res).unwrap();
    let (value, _) = bundle.format("hello-world", None).unwrap();
    assert_eq!(&value, "Hello, world!");
}
