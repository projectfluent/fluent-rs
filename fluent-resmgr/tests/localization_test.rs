use fluent_fallback::SyncLocalization;
use fluent_resmgr::resource_manager::ResourceManager;
use unic_langid::langid;

#[test]
fn localization_format_value() {
    let res_mgr = ResourceManager::new("./tests/resources/{locale}/{res_id}".into());

    let loc = SyncLocalization::with_generator(vec!["test.ftl".into()], res_mgr);

    let value = loc.format_value_sync("hello-world", None);
    assert_eq!(value, "Hello World");

    let value2 = loc.format_value_sync("new-message", None);
    assert_eq!(value2, "Nowa Wiadomość");

    let value3 = loc.format_value_sync("missing-message", None);
    assert_eq!(value3, "missing-message");
}

#[test]
fn resmgr_get_bundle() {
    let res_mgr = ResourceManager::new("./tests/resources/{locale}/{res_id}".into());

    let bundle = res_mgr.get_bundle(vec![langid!("en-US")], vec!["test.ftl".into()]);

    let mut errors = vec![];
    let msg = bundle.get_message("hello-world").expect("Message exists");
    let pattern = msg.value.expect("Message has a value");
    let value = bundle.format_pattern(&pattern, None, &mut errors);
    assert_eq!(value, "Hello World");
}
