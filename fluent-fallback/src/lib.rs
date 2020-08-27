use std::borrow::Borrow;
use std::borrow::Cow;
use std::path::PathBuf;

use fluent_bundle::FluentResource;
use fluent_bundle::{FluentArgs, FluentBundle};

use reiterate::Reiterate;

struct FluentBundleIteratorSync<R, I>
where
    I: Iterator<Item = FluentBundle<R>>,
{
    iter: I,
}

impl<R, I> Iterator for FluentBundleIteratorSync<R, I>
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
    pub resource_ids: Vec<PathBuf>,
    bundles: Reiterate<FluentBundleIteratorSync<R, I>>,
    generate_bundles: Box<dyn FnMut(Vec<PathBuf>) -> FluentBundleIteratorSync<R, I> + 'loc>,
}

impl<'loc, R, I> Localization<'loc, R, I>
where
    I: Iterator<Item = FluentBundle<R>>,
{
    pub fn new<F>(resource_ids: Vec<PathBuf>, mut generate_bundles: F) -> Self
    where
        F: FnMut(Vec<PathBuf>) -> I + 'loc,
    {
        let mut generate = move |x: Vec<PathBuf>| FluentBundleIteratorSync {
            iter: generate_bundles(x),
        };
        let bundles = Reiterate::new(generate(resource_ids.clone()));
        Localization {
            resource_ids,
            bundles,
            generate_bundles: Box::new(generate),
        }
    }

    pub fn on_change(&mut self) {
        self.bundles = Reiterate::new((self.generate_bundles)(self.resource_ids.clone()));
    }

    pub fn format_value<'l>(&'l self, id: &'l str, args: Option<&'l FluentArgs>) -> Cow<'l, str>
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
