use fluent_bundle::resource::FluentResource;

#[test]
fn resource_try_new() {
    let res = FluentResource::try_new("key = Value".into());
    assert_eq!(res.is_ok(), true);

    let res_err = FluentResource::try_new("2key = Value".into());
    assert_eq!(res_err.is_err(), true);
}
