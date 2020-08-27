use std::path::{Path, PathBuf};

use fluent_fallback::Localization;
use l10nregistry::registry::L10nRegistry;
use l10nregistry::source::FileSource;
use unic_langid::langid;

#[test]
fn localization_format_sync() {
    let resource_ids: Vec<PathBuf> = vec!["test.ftl".into(), "test2.ftl".into()];
    let pl = langid!("pl");
    let en_us = langid!("en-US");
    let locales = vec![&pl, &en_us];

    let main_fs = FileSource::new(
        "main".to_string(),
        vec![pl.clone(), en_us.clone()],
        "./tests/resources/{locale}/".into(),
    );

    let mut reg = L10nRegistry::new();

    reg.register_sources(vec![main_fs]).unwrap();

    let res_ids: Vec<&Path> = resource_ids.iter().map(|res_id| res_id.as_path()).collect();
    let generate_messages = |_res_ids: Vec<PathBuf>| reg.generate_bundles_sync(&locales, &res_ids);

    let loc = Localization::new(resource_ids.clone(), generate_messages);

    let value = loc.format_value("hello-world", None);
    assert_eq!(value, "Hello World [pl]");

    let value = loc.format_value("missing-message", None);
    assert_eq!(value, "missing-message");

    let value = loc.format_value("hello-world-3", None);
    assert_eq!(value, "Hello World 3 [en]");
}

#[test]
fn localization_format_async() {
    // let resource_ids: Vec<PathBuf> = vec!["test.ftl".into(), "test2.ftl".into()];
    // let pl = langid!("pl");
    // let en_us = langid!("en-US");
    // let locales = vec![&pl, &en_us];
    //
    // let main_fs = FileSource::new("main".to_string(), vec![pl.clone(), en_us.clone()], "./tests/resources/{locale}/".into());
    //
    // let mut reg = L10nRegistry::new();
    //
    // reg.register_sources(vec![main_fs]).unwrap();
    //
    // let res_ids: Vec<&Path> = resource_ids.iter().map(|res_id| res_id.as_path()).collect();
    // let generate_messages = |_res_ids: Vec<PathBuf>| {
    //     reg.generate_bundles(&locales, &res_ids)
    // };
    //
    // let loc = Localization::new(resource_ids.clone(), generate_messages);
    //
    // let value = loc.format_value("hello-world", None);
    // assert_eq!(value, "Hello World [pl]");
    //
    // let value = loc.format_value("missing-message", None);
    // assert_eq!(value, "missing-message");
    //
    // let value = loc.format_value("hello-world-3", None);
    // assert_eq!(value, "Hello World 3 [en]");
}
