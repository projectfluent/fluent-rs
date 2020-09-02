use fluent_bundle::FluentBundle;
use std::path::PathBuf;

use reiterate::Reiterate;

pub type BundleIterator<R> = dyn Iterator<Item = Box<FluentBundle<R>>>;

pub struct Localization<R> {
    pub resource_ids: Vec<PathBuf>,
    bundles: Reiterate<Box<BundleIterator<R>>>,
    generate_bundles_sync: Option<Box<dyn FnMut(Vec<PathBuf>) -> Box<BundleIterator<R>>>>,
}

impl<R> Localization<R> {
    pub fn new<F: 'static>(resource_ids: Vec<PathBuf>, mut generate_bundles_sync: F) -> Self
    where
        F: FnMut(Vec<PathBuf>) -> Box<BundleIterator<R>>,
    {
        let bundles = Reiterate::new(generate_bundles_sync(resource_ids.clone()));

        Self {
            resource_ids,
            bundles,
            generate_bundles_sync: Some(Box::new(generate_bundles_sync)),
        }
    }
}
