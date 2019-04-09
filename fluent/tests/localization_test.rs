use fluent::Localization;
use fluent_bundle::FluentBundle;
use fluent_bundle::resource::FluentResource;
use elsa::FrozenMap;

use std::io;
use std::fs;
use std::collections::HashMap;

fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

#[test]
fn localization_test() {
    let mut resources: FrozenMap<String, Box<FluentResource>> = FrozenMap::new();

    let resource_ids: Vec<String> = vec!["test.ftl".into(), "test2.ftl".into()];

    let generate_messages = |res_ids: &[String]| {
        let resources = &mut resources;
        let locales = vec!["en-US"];

        let mut bundles = vec![];

        for locale in locales {
            let mut bundle = FluentBundle::new(&[locale]);
            for res_id in res_ids {
                let res = if let Some(res) = resources.get(res_id) {
                    res
                } else {
                    let source = read_file(res_id).unwrap();
                    let res = FluentResource::try_new(source).unwrap();
                    resources.insert(res_id.to_string(), Box::new(res))
                };
                bundle.add_resource(&res);
            }
            bundles.push(bundle);
        }

        return bundles.into_iter();
    };

    let mut loc = Localization::new(resource_ids, generate_messages);

    let value = loc.format_value("hello-world", None);
    assert_eq!(value, "hello-world");
}
