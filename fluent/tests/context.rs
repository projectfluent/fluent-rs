extern crate fluent;

use self::fluent::context::MessageContext;

#[test]
fn context_new_from_str() {
    let arr_of_str = ["x-testing"];
    let _ = MessageContext::new(&arr_of_str);
    let _ = MessageContext::new(&arr_of_str[..]);

    let vec_of_str = vec!["x-testing"];
    let _ = MessageContext::new(&vec_of_str);
    let _ = MessageContext::new(&vec_of_str[..]);

    let iter_of_str = ["x-testing"].iter();
    let vec_from_iter = iter_of_str.cloned().collect::<Vec<_>>();
    let _ = MessageContext::new(&vec_from_iter);
    let _ = MessageContext::new(&vec_from_iter[..]);
}

#[test]
fn context_new_from_strings() {
    let arr_of_strings = ["x-testing".to_string()];
    let arr_of_str = [arr_of_strings[0].as_str()];

    let _ = MessageContext::new(&arr_of_str);
    let _ = MessageContext::new(&arr_of_str[..]);

    let vec_of_strings = ["x-testing".to_string()];
    let vec_of_str = [vec_of_strings[0].as_str()];

    let _ = MessageContext::new(&vec_of_str);
    let _ = MessageContext::new(&vec_of_str[..]);

    let iter_of_strings = arr_of_strings.iter();
    let vec_from_iter = iter_of_strings
        .map(|elem| elem.as_str())
        .collect::<Vec<_>>();
    let _ = MessageContext::new(&vec_from_iter);
    let _ = MessageContext::new(&vec_from_iter[..]);
}
