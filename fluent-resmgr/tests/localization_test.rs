use fluent_fallback::Localization;
use fluent_resmgr::resource_manager::ResourceManager;
use std::borrow::Cow;
use unic_langid::langid;

#[test]
fn localization_format_value() {
    let res_mgr = ResourceManager::new("./tests/resources/{locale}/{res_id}".into());

    let loc = Localization::with_env(
        vec!["test.ftl".into()],
        true,
        vec!["en-US".parse().unwrap(), "pl".parse().unwrap()],
        res_mgr,
    );
    let bundles = loc.bundles();
    let mut errors = vec![];

    let value = bundles
        .format_value_sync("hello-world", None, &mut errors)
        .unwrap();
    assert_eq!(value, Some(Cow::Borrowed("Hello World")));

    let value2 = bundles
        .format_value_sync("new-message", None, &mut errors)
        .unwrap();
    assert_eq!(value2, Some(Cow::Borrowed("Nowa Wiadomość")));

    let value3 = bundles
        .format_value_sync("missing-message", None, &mut errors)
        .unwrap();
    assert_eq!(value3, None);
}

#[test]
fn resmgr_get_bundle() {
    let res_mgr = ResourceManager::new("./tests/resources/{locale}/{res_id}".into());

    let bundle = res_mgr.get_bundle(vec![langid!("en-US")], vec!["test.ftl".into()]);

    let mut errors = vec![];
    let msg = bundle.get_message("hello-world").expect("Message exists");
    let pattern = msg.value().expect("Message has a value");
    let value = bundle.format_pattern(&pattern, None, &mut errors);
    assert_eq!(value, "Hello World");
}
