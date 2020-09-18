use fluent_fallback::Localization;
use fluent_resmgr::resource_manager::ResourceManager;
use unic_langid::langid;

#[test]
fn localization_format_value() {
    let res_mgr = ResourceManager::new("./tests/resources/{locale}/{res_id}".into());

    let generate_messages = |res_ids: &[String]| {
        res_mgr.get_bundles(vec![langid!("en-US"), langid!("pl")], res_ids.to_vec())
    };

    let loc = Localization::new(vec!["test.ftl".into()], generate_messages);

    let value = loc.format_value("hello-world", None);
    assert_eq!(value, "Hello World");

    let value2 = loc.format_value("new-message", None);
    assert_eq!(value2, "Nowa Wiadomość");

    let value3 = loc.format_value("missing-message", None);
    assert_eq!(value3, "missing-message");
}

#[test]
fn resmgr_get_bundle() {
    let res_mgr = ResourceManager::new("./tests/resources/{locale}/{res_id}".into());

    let bundle = res_mgr.get_bundle(vec![langid!("en-US")], vec!["test.ftl".into()]);

    let mut errors = vec![];
    let msg = bundle.get_message("hello-world").expect("Message exists");
    let pattern = msg.value.expect("Message has a value");
    let value = bundle.format_pattern_to_string(&pattern, None, &mut errors);
    assert_eq!(value, "Hello World");
}
