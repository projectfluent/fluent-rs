use std::sync::Arc;

use fluent_bundle::FluentBundle;
use fluent_bundle::FluentResource;

#[test]
fn bundle_new_from_str() {
    let arr_of_str = ["x-testing"];
    let _: FluentBundle<FluentResource> = FluentBundle::new(&arr_of_str);
    let _: FluentBundle<FluentResource> = FluentBundle::new(&arr_of_str[..]);

    let vec_of_str = vec!["x-testing"];
    let _: FluentBundle<FluentResource> = FluentBundle::new(&vec_of_str);
    let _: FluentBundle<FluentResource> = FluentBundle::new(&vec_of_str[..]);

    let iter_of_str = ["x-testing"].iter();
    let vec_from_iter = iter_of_str.cloned().collect::<Vec<_>>();
    let _: FluentBundle<FluentResource> = FluentBundle::new(&vec_from_iter);
    let _: FluentBundle<FluentResource> = FluentBundle::new(&vec_from_iter[..]);
}

#[test]
fn bundle_new_from_strings() {
    let arr_of_strings = ["x-testing".to_string()];
    let arr_of_str = [arr_of_strings[0].as_str()];

    let _: FluentBundle<FluentResource> = FluentBundle::new(&arr_of_str);
    let _: FluentBundle<FluentResource> = FluentBundle::new(&arr_of_str[..]);

    let vec_of_strings = ["x-testing".to_string()];
    let vec_of_str = [vec_of_strings[0].as_str()];

    let _: FluentBundle<FluentResource> = FluentBundle::new(&vec_of_str);
    let _: FluentBundle<FluentResource> = FluentBundle::new(&vec_of_str[..]);

    let iter_of_strings = arr_of_strings.iter();
    let vec_from_iter = iter_of_strings
        .map(|elem| elem.as_str())
        .collect::<Vec<_>>();
    let _: FluentBundle<FluentResource> = FluentBundle::new(&vec_from_iter);
    let _: FluentBundle<FluentResource> = FluentBundle::new(&vec_from_iter[..]);
}

fn create_bundle<'a, 'b>(locales: &'b Vec<&'b str>) -> FluentBundle<FluentResource> {
    FluentBundle::new(locales)
}

#[test]
fn bundle_locale_diff_scope() {
    let locales = vec!["x-testing"];
    create_bundle(&locales);
}

#[test]
fn bundle_new_owned() {
    let locs = ["x-testing"];

    let mut bundle = FluentBundle::new(&locs);

    let res = FluentResource::try_new("key = Value".to_string()).unwrap();
    let res2 = FluentResource::try_new("key2 = Value 2".to_string()).unwrap();

    bundle.add_resource(res).expect("Added a resource");
    bundle.add_resource(res2).expect("Added a resource");
}

#[test]
fn bundle_new_borrowed() {
    let locs = ["x-testing"];

    let mut bundle = FluentBundle::new(&locs);

    let res = FluentResource::try_new("key = Value".to_string()).unwrap();
    let res2 = FluentResource::try_new("key2 = Value 2".to_string()).unwrap();

    bundle.add_resource(&res).expect("Added a resource");
    bundle.add_resource(&res2).expect("Added a resource");
}

#[test]
fn bundle_new_arc() {
    let locs = ["x-testing"];

    let mut bundle = FluentBundle::new(&locs);

    let res = FluentResource::try_new("key = Value".to_string()).unwrap();
    let res2 = FluentResource::try_new("key2 = Value 2".to_string()).unwrap();

    bundle
        .add_resource(Arc::new(res))
        .expect("Added a resource");
    bundle
        .add_resource(Arc::new(res2))
        .expect("Added a resource");
}
