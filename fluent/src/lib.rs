use std::collections::HashMap;

pub use fluent_bundle::FluentBundle;
pub use fluent_bundle::FluentResource;
pub use fluent_bundle::FluentValue;

use reiterate::Reiterate;

struct FluentBundleIterator<'loc> {
    iter: Box<Iterator<Item = FluentBundle<'loc>> + 'loc>,
}

impl<'loc> Iterator for FluentBundleIterator<'loc> {
    type Item = Box<FluentBundle<'loc>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Box::new)
    }
}

pub struct Localization<'loc> {
    pub resource_ids: Vec<String>,
    bundles: Reiterate<FluentBundleIterator<'loc>>,
    generate_bundles: Box<FnMut(&[String]) -> FluentBundleIterator<'loc> + 'loc>,
}

impl<'loc> Localization<'loc> {
    pub fn new<F, I>(resource_ids: Vec<String>, mut generate_bundles: F) -> Self
    where
        F: FnMut(&[String]) -> I + 'loc,
        I: Iterator<Item = FluentBundle<'loc>> + 'loc,
    {
        let mut generate2 = move |x: &[String]| FluentBundleIterator {
            iter: Box::new(generate_bundles(x)),
        };
        let bundles = Reiterate::new(generate2(&resource_ids));
        Localization {
            resource_ids,
            bundles: bundles,
            generate_bundles: Box::new(generate2),
        }
    }

    pub fn on_change(&mut self) {
        self.bundles = Reiterate::new((self.generate_bundles)(&self.resource_ids));
    }

    pub fn format_value(&mut self, id: &str, args: Option<&HashMap<&str, FluentValue>>) -> String {
        for bundle in &self.bundles {
            if bundle.has_message(id) {
                let res = bundle.format(id, args).unwrap();
                return res.0.to_string();
            }
        }
        return id.into();
    }
}
