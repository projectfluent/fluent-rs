use std::borrow::Borrow;
use std::collections::HashMap;

use fluent_bundle::FluentBundle;
use fluent_bundle::FluentResource;
use fluent_bundle::FluentValue;

use reiterate::Reiterate;

struct FluentBundleIterator<'loc, R> {
    iter: Box<dyn Iterator<Item = FluentBundle<'loc, R>> + 'loc>,
}

impl<'loc, R> Iterator for FluentBundleIterator<'loc, R> {
    type Item = Box<FluentBundle<'loc, R>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Box::new)
    }
}

pub struct Localization<'loc, R> {
    pub resource_ids: Vec<String>,
    bundles: Reiterate<FluentBundleIterator<'loc, R>>,
    generate_bundles: Box<dyn FnMut(&[String]) -> FluentBundleIterator<'loc, R> + 'loc>,
}

impl<'loc, R> Localization<'loc, R> {
    pub fn new<F, I>(resource_ids: Vec<String>, mut generate_bundles: F) -> Self
    where
        F: FnMut(&[String]) -> I + 'loc,
        I: Iterator<Item = FluentBundle<'loc, R>> + 'loc,
    {
        let mut generate2 = move |x: &[String]| FluentBundleIterator {
            iter: Box::new(generate_bundles(x)),
        };
        let bundles = Reiterate::new(generate2(&resource_ids));
        Localization {
            resource_ids,
            bundles,
            generate_bundles: Box::new(generate2),
        }
    }

    pub fn on_change(&mut self) {
        self.bundles = Reiterate::new((self.generate_bundles)(&self.resource_ids));
    }

    pub fn format_value(&mut self, id: &str, args: Option<&HashMap<&str, FluentValue>>) -> String
    where
        R: Borrow<FluentResource>,
    {
        for bundle in &self.bundles {
            if bundle.has_message(id) {
                let res = bundle.format(id, args).unwrap();
                return res.0.to_string();
            }
        }
        id.into()
    }
}
