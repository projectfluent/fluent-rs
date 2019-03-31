use std::collections::HashMap;

pub use fluent_bundle::FluentBundle;
pub use fluent_bundle::FluentResource;
pub use fluent_bundle::FluentValue;

pub trait FluentBundleIterator<'l>: Iterator<Item = FluentBundle<'l>> {}

pub type Fbi<'l> = Box<FluentBundleIterator<'l> + 'l>;

pub struct Localization<'l> {
    pub resource_ids: Vec<String>,
    bundles: Fbi<'l>,
    generate_bundles: Box<FnMut(&[String]) -> Fbi<'l> + 'l>,
}

impl<'l> Localization<'l> {
    pub fn new<F, S: Into<String>>(resource_ids: Vec<S>, mut generate_bundles: F) -> Self
    where F: FnMut(&[String]) -> Fbi<'l> + 'l {
        let res_ids: Vec<String> = resource_ids.into_iter().map(|res| res.into()).collect();
        let bundles = generate_bundles(&res_ids);
        Localization {
            resource_ids: res_ids,
            bundles: bundles,
            generate_bundles: Box::new(generate_bundles),
        }
    }

    pub fn on_change(&mut self) {
        self.bundles = (self.generate_bundles)(&self.resource_ids);
    }

    pub fn format_value(&mut self, id: &str, args: Option<&HashMap<&str, FluentValue>>) -> String {
        //let bundles = self.bundles.get_or_insert_with(|| {
            //(self.generate_bundles)(&self.resource_ids)
        //});
        for bundle in &*self.bundles {
            if bundle.has_message(id) {
                let res = bundle.format(id, args).unwrap();
                return res.0;
            }
        }
        return id.into();
    }
}
