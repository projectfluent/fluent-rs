use fluent_bundle::{FluentBundle, FluentResource};
use fluent_fallback::Localization;
use l10nregistry::registry::L10nRegistry;
use l10nregistry::source::FileSource;
use std::path::Path;
use std::rc::Rc;
use unic_langid::{langid, LanguageIdentifier};

const LOCALES: &[LanguageIdentifier] = &[langid!("pl"), langid!("en-US")];

fn get_app_locales() -> &'static [LanguageIdentifier] {
    LOCALES
}

type ResRc = Rc<FluentResource>;

fn get_new_localization<'l>(reg: &'l L10nRegistry, res_ids: &'l [&'l str]) -> Localization<
    'l,
    ResRc,
    impl Iterator<Item = FluentBundle<ResRc>> + 'l,
    &'l &'l str,
    impl Iterator<Item = &'l &'l str> + Clone + 'l>
 {

    let loc = Localization::new(res_ids.iter(), move |res_ids| {
        let locales = get_app_locales();
        Box::new(reg.generate_bundles_sync(locales, res_ids))
    });
    loc
}

#[test]
fn localization_format_sync() {
    let resource_ids = vec!["test.ftl", "test2.ftl"];

    let mut reg = L10nRegistry::new();

    let main_fs = FileSource::new(
        "main".to_string(),
        get_app_locales().to_vec(),
        "./tests/resources/{locale}/".into(),
    );

    reg.register_sources(vec![main_fs]).unwrap();

    let loc = get_new_localization(&reg, &resource_ids);

    let value = loc.format_value_sync("hello-world", None);
    assert_eq!(value, "Hello World [pl]");

    let value = loc.format_value_sync("missing-message", None);
    assert_eq!(value, "missing-message");

    let value = loc.format_value_sync("hello-world-3", None);
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
