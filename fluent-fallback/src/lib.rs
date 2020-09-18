use std::borrow::Borrow;
use std::borrow::Cow;

use fluent_bundle::FluentResource;
use fluent_bundle::{FluentArgs, FluentBundle};

use reiterate::Reiterate;

struct FluentBundleIterator<R, I>
where
    I: Iterator<Item = FluentBundle<R>>,
{
    iter: I,
}

impl<R, I> Iterator for FluentBundleIterator<R, I>
where
    I: Iterator<Item = FluentBundle<R>>,
{
    type Item = Box<FluentBundle<R>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Box::new)
    }
}

pub struct Localization<'loc, R, I>
where
    I: Iterator<Item = FluentBundle<R>> + 'loc,
{
    pub resource_ids: Vec<String>,
    bundles: Reiterate<FluentBundleIterator<R, I>>,
    generate_bundles: Box<dyn FnMut(&[String]) -> FluentBundleIterator<R, I> + 'loc>,
}

impl<'loc, R, I> Localization<'loc, R, I>
where
    I: Iterator<Item = FluentBundle<R>>,
{
    pub fn new<F>(resource_ids: Vec<String>, mut generate_bundles: F) -> Self
    where
        F: FnMut(&[String]) -> I + 'loc,
    {
        let mut generate = move |x: &[String]| FluentBundleIterator {
            iter: generate_bundles(x),
        };
        let bundles = Reiterate::new(generate(&resource_ids));
        Localization {
            resource_ids,
            bundles,
            generate_bundles: Box::new(generate),
        }
    }

    pub fn on_change(&mut self) {
        self.bundles = Reiterate::new((self.generate_bundles)(&self.resource_ids));
    }

    pub fn format_value<'l>(&'l self, id: &'l str, args: Option<&'l FluentArgs>) -> Cow<'l, str>
    where
        R: Borrow<FluentResource>,
    {
        for bundle in &self.bundles {
            if let Some(msg) = bundle.get_message(id) {
                if let Some(pattern) = msg.value {
                    let mut errors = vec![];
                    return bundle.format_pattern_to_string(pattern, args, &mut errors);
                }
            }
        }
        id.into()
    }
}
