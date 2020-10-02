use fluent_bundle::FluentResource;
use fluent_fallback::{L10nKey, Localization};
use l10nregistry::registry::L10nRegistry;
use l10nregistry::source::FileSource;
use std::borrow::Cow;
use std::rc::Rc;
use unic_langid::{langid, LanguageIdentifier};

static LOCALES: &[LanguageIdentifier] = &[langid!("pl"), langid!("en-US")];
static mut L10N_REGISTRY: Option<L10nRegistry> = None;

fn fetch_sync(path: &str) -> Result<Option<String>, std::io::Error> {
    if !std::path::Path::new(path).exists() {
        return Ok(None);
    }
    Ok(Some(std::fs::read_to_string(path)?))
}

fn get_l10n_registry() -> &'static L10nRegistry {
    let reg: &mut Option<L10nRegistry> = unsafe { &mut L10N_REGISTRY };

    reg.get_or_insert_with(|| {
        let mut reg = L10nRegistry::new();

        let main_fs = FileSource::new(
            "main".to_string(),
            get_app_locales().to_vec(),
            "./tests/resources/{locale}/".into(),
            fetch_sync,
        );

        reg.register_sources(vec![main_fs]).unwrap();
        reg
    })
}

fn get_app_locales() -> &'static [LanguageIdentifier] {
    LOCALES
}

type ResRc = Rc<FluentResource>;

fn get_new_localization(reg: &'static L10nRegistry, res_ids: Vec<String>) -> Localization<ResRc> {
    let loc = Localization::new(res_ids, move |res_ids| {
        let locales = get_app_locales();
        Box::new(
            reg.generate_bundles_sync(locales, res_ids)
                .map(|bundle| Box::new(bundle)),
        )
    });
    loc
}

#[test]
fn localization_format_value_sync() {
    let resource_ids = vec!["test.ftl".into(), "test2.ftl".into()];

    let reg = get_l10n_registry();

    let loc = get_new_localization(reg, resource_ids);

    let value = loc.format_value_sync("hello-world", None);
    assert_eq!(value, "Hello World [pl]");

    let value = loc.format_value_sync("missing-message", None);
    assert_eq!(value, "missing-message");

    let value = loc.format_value_sync("hello-world-3", None);
    assert_eq!(value, "Hello World 3 [en]");
}

#[test]
fn localization_format_values_sync() {
    let resource_ids = vec!["test.ftl".into(), "test2.ftl".into()];

    let reg = get_l10n_registry();

    let loc = get_new_localization(reg, resource_ids);

    let keys = vec![
        L10nKey {
            id: "hello-world".to_string(),
            args: None,
        },
        L10nKey {
            id: "missing-message".to_string(),
            args: None,
        },
        L10nKey {
            id: "hello-world-3".to_string(),
            args: None,
        },
    ];
    let values = loc.format_values_sync(&keys);
    assert_eq!(values.len(), 3);
    assert_eq!(values[0], Some(Cow::Borrowed("Hello World [pl]")));
    assert_eq!(values[1], None);
    assert_eq!(values[2], Some(Cow::Borrowed("Hello World 3 [en]")));
}

#[test]
fn localization_format_messages_sync() {
    let resource_ids = vec!["test.ftl".into(), "test2.ftl".into()];

    let reg = get_l10n_registry();

    let loc = get_new_localization(reg, resource_ids);

    let keys = vec![
        L10nKey {
            id: "message-1".to_string(),
            args: None,
        },
        L10nKey {
            id: "message-2".to_string(),
            args: None,
        },
    ];
    let messages = loc.format_messages_sync(&keys);
    assert_eq!(messages.len(), 2);
    assert_eq!(
        messages[0].as_ref().unwrap().value,
        Some("Message 1 Value [en]".to_string())
    );
    assert_eq!(
        messages[0].as_ref().unwrap().attributes[0].value,
        "Message 1 Attribute [en]".to_string()
    );
    assert_eq!(
        messages[1].as_ref().unwrap().value,
        Some("Message 2 Value [en]".to_string())
    );
    assert_eq!(
        messages[1].as_ref().unwrap().attributes[0].value,
        "Message 2 Attribute [en]".to_string()
    );
}

#[test]
fn localization_format_value_async() {
    // let resource_ids: Vec<String> = vec!["test.ftl".into(), "test2.ftl".into()];
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
    // let generate_messages = |_res_ids: Vec<String>| {
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
