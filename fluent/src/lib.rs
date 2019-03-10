pub use fluent_bundle::FluentBundle;
pub use fluent_bundle::FluentResource;
pub use fluent_bundle::FluentValue;

pub trait FluentBundleIterator<'l>: Iterator<Item = FluentBundle<'l>> {}

pub type Fbi<'l> = Box<FluentBundleIterator<'l>>;

pub struct Localization<'l> {
    pub resource_ids: Vec<String>,
    pub bundles: Option<Fbi<'l>>,
    generate_bundles: Box<Fn() -> Fbi<'l> + 'l>,
}

impl<'l> Localization<'l> {
    pub fn new<F>(resource_ids: Vec<String>, generate_bundles: F) -> Self
    where F: Fn() -> Fbi<'l> + 'l {
        Localization {
            resource_ids,
            bundles: None,
            generate_bundles: Box::new(generate_bundles),
        }
    }

    pub fn on_change(&mut self) {
        self.bundles = Some((self.generate_bundles)());
    }
}
