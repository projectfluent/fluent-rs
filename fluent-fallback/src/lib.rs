use std::borrow::Borrow;
use std::borrow::Cow;
use std::path::Path;

use reiterate::Reiterate;

use fluent_bundle::FluentResource;
use fluent_bundle::{FluentArgs, FluentBundle};

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

pub struct Localization<'loc, R, I, P, L>
where
    P: AsRef<Path>,
    L: Iterator<Item = P> + Clone,
    I: Iterator<Item = FluentBundle<R>> + 'loc,
{
    pub resource_ids: L,
    bundles: Reiterate<FluentBundleIterator<R, I>>,
    generate_bundles_sync: Option<Box<dyn FnMut(L) -> FluentBundleIterator<R, I> + 'loc>>,
}

impl<'loc, R, I, P, L> Localization<'loc, R, I, P, L>
where
    I: Iterator<Item = FluentBundle<R>>,
    P: AsRef<Path>,
    L: Iterator<Item = P> + Clone,
{
    pub fn new<F>(resource_ids: L, mut generate_bundles_sync: F) -> Self
    where
        F: FnMut(L) -> I + 'loc,
    {
        let mut generate = move |x: L| FluentBundleIterator {
            iter: generate_bundles_sync(x),
        };
        let bundles = Reiterate::new(generate(resource_ids.clone()));
        Localization {
            resource_ids,
            bundles,
            generate_bundles_sync: Some(Box::new(generate)),
        }
    }

    pub fn on_change(&mut self) {
        self.bundles = Reiterate::new((self.generate_bundles_sync.as_mut().unwrap())(
            self.resource_ids.clone(),
        ));
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
