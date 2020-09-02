use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use std::borrow::Borrow;
use std::borrow::Cow;
use std::path::PathBuf;

use reiterate::Reiterate;

pub type BundleIterator<R> = dyn Iterator<Item = Box<FluentBundle<R>>>;

pub struct Localization<R> {
    pub resource_ids: Vec<PathBuf>,
    bundles: Reiterate<Box<BundleIterator<R>>>,
    generate_bundles_sync: Box<dyn FnMut(Vec<PathBuf>) -> Box<BundleIterator<R>>>,
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
            generate_bundles_sync: Box::new(generate_bundles_sync),
        }
    }

    pub fn on_change(&mut self) {
        self.bundles = Reiterate::new((self.generate_bundles_sync)(self.resource_ids.clone()));
    }

    pub fn format_value_sync<'l>(
        &'l self,
        id: &'l str,
        args: Option<&'l FluentArgs>,
    ) -> Cow<'l, str>
    where
        R: Borrow<FluentResource>,
    {
        for bundle in &self.bundles {
            if let Some(msg) = bundle.get_message(id) {
                if let Some(pattern) = msg.value {
                    let mut errors = vec![];
                    return bundle.format_pattern(pattern, args, &mut errors);
                }
            }
        }
        id.into()
    }
}
