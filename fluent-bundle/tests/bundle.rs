use fluent_bundle::bundle::FluentBundle;

#[test]
fn bundle_new_from_str() {
    let arr_of_str = ["x-testing"];
    let _ = FluentBundle::new(&arr_of_str);
    let _ = FluentBundle::new(&arr_of_str[..]);

    let vec_of_str = vec!["x-testing"];
    let _ = FluentBundle::new(&vec_of_str);
    let _ = FluentBundle::new(&vec_of_str[..]);

    let iter_of_str = ["x-testing"].iter();
    let vec_from_iter = iter_of_str.cloned().collect::<Vec<_>>();
    let _ = FluentBundle::new(&vec_from_iter);
    let _ = FluentBundle::new(&vec_from_iter[..]);
}

#[test]
fn bundle_new_from_strings() {
    let arr_of_strings = ["x-testing".to_string()];
    let arr_of_str = [arr_of_strings[0].as_str()];

    let _ = FluentBundle::new(&arr_of_str);
    let _ = FluentBundle::new(&arr_of_str[..]);

    let vec_of_strings = ["x-testing".to_string()];
    let vec_of_str = [vec_of_strings[0].as_str()];

    let _ = FluentBundle::new(&vec_of_str);
    let _ = FluentBundle::new(&vec_of_str[..]);

    let iter_of_strings = arr_of_strings.iter();
    let vec_from_iter = iter_of_strings
        .map(|elem| elem.as_str())
        .collect::<Vec<_>>();
    let _ = FluentBundle::new(&vec_from_iter);
    let _ = FluentBundle::new(&vec_from_iter[..]);
}

fn create_bundle<'a, 'b>(locales: &'b Vec<&'b str>) -> FluentBundle<'a> {
    FluentBundle::new(locales)
}

#[test]
fn bundle_locale_diff_scope() {
    let locales = vec!["x-testing"];
    create_bundle(&locales);
}
