use fluent::Localization;
use fluent_resmgr::resource_manager::ResourceManager;

#[test]
fn localization_basic() {
    let res_mgr = ResourceManager::new();

    let generate_messages = |res_ids: &[String]| res_mgr.get_bundles(vec!["en-US".into()], res_ids.to_vec());

    let _loc = Localization::new(vec!["test.ftl".into()], generate_messages);
}
