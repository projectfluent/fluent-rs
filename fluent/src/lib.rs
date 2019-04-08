use std::collections::HashMap;

pub use fluent_bundle::FluentBundle;
pub use fluent_bundle::FluentResource;
pub use fluent_bundle::FluentValue;

// pub trait FluentBundleIterator<'l>: > {}

struct FluentBundleIterator<'l> {
    iter: Box<Iterator<Item = FluentBundle<'l>> + 'l>,
}

impl<'a, 'l> Iterator for &'a mut FluentBundleIterator<'l> {
    type Item = FluentBundle<'l>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct Localization<'l> {
    pub resource_ids: Vec<String>,
    bundles: FluentBundleIterator<'l>,
    generate_bundles: Box<FnMut(&[String]) -> FluentBundleIterator<'l> + 'l>,
}

impl<'l> Localization<'l> {
    pub fn new<F, S: Into<String>, I>(resource_ids: Vec<S>, mut generate_bundles: F) -> Self
    where
        F: FnMut(&[String]) -> I + 'l,
        I: Iterator<Item = FluentBundle<'l>> + 'l,
    {
        let res_ids: Vec<String> = resource_ids.into_iter().map(|res| res.into()).collect();
        let mut generate2 = move |x: &[String]| FluentBundleIterator {
            iter: Box::new(generate_bundles(x)),
        };
        let bundles = generate2(&res_ids);
        Localization {
            resource_ids: res_ids,
            bundles: bundles,
            generate_bundles: Box::new(generate2),
        }
    }

    pub fn on_change(&mut self) {
        self.bundles = (self.generate_bundles)(&self.resource_ids);
    }

    pub fn format_value(&mut self, id: &str, args: Option<&HashMap<&str, FluentValue>>) -> String {
        //let bundles = self.bundles.get_or_insert_with(|| {
        //(self.generate_bundles)(&self.resource_ids)
        //});
        for bundle in &mut self.bundles {
            if bundle.has_message(id) {
                let res = bundle.format(id, args).unwrap();
                return res.0;
            }
        }
        return id.into();
    }
}
