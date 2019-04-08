use fluent::Localization;
use fluent_resmgr::resource_manager::ResourceManager;

#[test]
fn localization_format_value() {
    let res_mgr = ResourceManager::new("./tests/resources/{locale}/{res_id}".into());

    let generate_messages = |res_ids: &[String]| res_mgr.get_bundles(vec!["en-US".into(), "pl".into()], res_ids.to_vec());

    let mut loc = Localization::new(vec!["test.ftl".into()], generate_messages);

    let value = loc.format_value("hello-world", None);

    assert_eq!(value, "Hello World");

    let value2 = loc.format_value("new-message", None);

    assert_eq!(value2, "Nowa Wiadomość");
}
