use fluent_bundle::types::FluentNumber;
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use icu_locid::langid;

#[test]
fn test_function_resolve() {
    // 1. Create bundle
    let ftl_string = String::from(
        "
liked-count = { $num ->
        [0]     No likes yet.
        [one]   One person liked your message
       *[other] { $num } people liked your message
    }

liked-count2 = { NUMBER($num) ->
        [0]     No likes yet.
        [one]   One person liked your message
       *[other] { $num } people liked your message
    }
    ",
    );

    let res = FluentResource::try_new(ftl_string).expect("Could not parse an FTL string.");
    let mut bundle = FluentBundle::new(vec![langid!("en")]);

    bundle
        .add_function("NUMBER", |positional, named| match positional.first() {
            Some(FluentValue::Number(n)) => {
                let mut num = n.clone();
                num.options.merge(named);

                FluentValue::Number(num)
            }
            Some(FluentValue::String(s)) => {
                let num: f64 = if let Ok(n) = s.clone().parse() {
                    n
                } else {
                    return FluentValue::Error;
                };
                let mut num = FluentNumber {
                    value: num,
                    options: Default::default(),
                };
                num.options.merge(named);

                FluentValue::Number(num)
            }
            _ => FluentValue::Error,
        })
        .expect("Failed to add a function.");

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    // 2. Example without NUMBER call
    let msg = bundle
        .get_message("liked-count")
        .expect("Message doesn't exist.");

    let mut args = FluentArgs::new();
    args.set("num", FluentValue::from("1"));

    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("\u{2068}1\u{2069} people liked your message", &value);

    // 3. Example with passing number, but without NUMBER call
    let mut args = FluentArgs::new();
    args.set("num", FluentValue::from(1));

    let msg = bundle
        .get_message("liked-count")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("One person liked your message", &value);

    // 4. Example with NUMBER call
    let mut args = FluentArgs::new();
    args.set("num", FluentValue::from("1"));

    let msg = bundle
        .get_message("liked-count2")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("One person liked your message", &value);

    // 5. Example with NUMBER call from number
    let mut args = FluentArgs::new();
    args.set("num", FluentValue::from(1));

    let msg = bundle
        .get_message("liked-count2")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!("One person liked your message", &value);
}
