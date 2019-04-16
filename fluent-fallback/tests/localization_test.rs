use elsa::FrozenMap;
use fluent_bundle::resource::FluentResource;
use fluent_bundle::FluentBundle;
use fluent_fallback::Localization;

use std::cell::RefCell;
use std::fs;
use std::io;

fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

#[test]
fn localization_format() {
    let resources: FrozenMap<String, Box<FluentResource>> = FrozenMap::new();

    let resource_ids: Vec<String> = vec!["test.ftl".into(), "test2.ftl".into()];
    let res_path_scheme = "./tests/resources/{locale}/{res_id}";

    let generate_messages = |res_ids: &[String]| {
        let locales = vec!["pl", "en-US"];

        let mut bundles = vec![];

        for locale in locales {
            let mut bundle = FluentBundle::new(&[locale]);
            let res_path = res_path_scheme.replace("{locale}", locale);
            for res_id in res_ids {
                let path = res_path.replace("{res_id}", res_id);
                let res = if let Some(res) = resources.get(&path) {
                    res
                } else {
                    let source = read_file(&path).unwrap();
                    let res = FluentResource::try_new(source).unwrap();
                    resources.insert(path, Box::new(res))
                };
                bundle.add_resource(&res).unwrap();
            }
            bundles.push(bundle);
        }

        return bundles.into_iter();
    };

    let mut loc = Localization::new(resource_ids, generate_messages);

    let value = loc.format_value("hello-world", None);
    assert_eq!(value, "Hello World [pl]");

    let value = loc.format_value("missing-message", None);
    assert_eq!(value, "missing-message");

    let value = loc.format_value("hello-world-3", None);
    assert_eq!(value, "Hello World 3 [en]");
}

#[test]
fn localization_on_change() {
    let resources: FrozenMap<String, Box<FluentResource>> = FrozenMap::new();

    let resource_ids: Vec<String> = vec!["test.ftl".into(), "test2.ftl".into()];
    let res_path_scheme = "./tests/resources/{locale}/{res_id}";

    let available_locales = RefCell::new(vec![String::from("en-US")]);

    let generate_messages = |res_ids: &[String]| {
        let mut bundles = vec![];

        for locale in available_locales.borrow().iter() {
            let mut bundle = FluentBundle::new(&[locale]);
            let res_path = res_path_scheme.replace("{locale}", locale);
            for res_id in res_ids {
                let path = res_path.replace("{res_id}", res_id);
                let res = if let Some(res) = resources.get(&path) {
                    res
                } else {
                    let source = read_file(&path).unwrap();
                    let res = FluentResource::try_new(source).unwrap();
                    resources.insert(path, Box::new(res))
                };
                bundle.add_resource(&res).unwrap();
            }
            bundles.push(bundle);
        }

        return bundles.into_iter();
    };

    let mut loc = Localization::new(resource_ids, generate_messages);

    let value = loc.format_value("hello-world", None);
    assert_eq!(value, "Hello World [en]");

    available_locales.borrow_mut().insert(0, String::from("pl"));

    loc.on_change();

    let value = loc.format_value("hello-world", None);
    assert_eq!(value, "Hello World [pl]");
}
