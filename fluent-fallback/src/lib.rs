use std::borrow::Borrow;
use std::borrow::Cow;
use std::path::PathBuf;

use futures::stream::{Stream, StreamExt};

use fluent_bundle::FluentResource;
use fluent_bundle::{FluentArgs, FluentBundle};

pub struct Localization {}

impl Localization {
    pub fn new<I, R>(
        resource_ids: Vec<PathBuf>,
        // generate_bundles: Option<impl FnMut(Vec<PathBuf>) -> S>,
        generate_bundles_sync: Option<impl FnMut(&[PathBuf]) -> I>,
    ) -> Self
        where
            I: Iterator<Item = FluentBundle<R>>,
            // S: Stream<Item = FluentBundle<R>>
        {
        Self {
            // resource_ids,
            // generate_bundles,
            // generate_bundles_sync,
        }
    }
// pub struct Localization<R, I, S>
// where
//     I: Iterator<Item = FluentBundle<R>>,
//     S: Stream<Item = FluentBundle<R>>
// {
//     resource_ids: Vec<PathBuf>,
//     generate_bundles: Option<fn(Vec<PathBuf>) -> S>,
//     generate_bundles_sync: Option<fn(Vec<PathBuf>) -> I>,
// }
//
// impl<'l, R: 'l, I, S> Localization<R, I, S>
// where
//     I: Iterator<Item = FluentBundle<R>>,
//     S: Stream<Item = FluentBundle<R>>,
// {
//     pub fn new<F, A>(
//         resource_ids: Vec<PathBuf>,
//         generate_bundles: Option<fn(Vec<PathBuf>) -> S>,
//         generate_bundles_sync: Option<fn(Vec<PathBuf>) -> I>,
//     ) -> Self {
//         Self {
//             resource_ids,
//             generate_bundles,
//             generate_bundles_sync,
//         }
//     }

    // pub async fn format_value(&mut self, id: &'l str, args: Option<&'l FluentArgs<'_>>) -> Cow<'l, str>
    // where
    //     R: Borrow<FluentResource>,
    // {
    //     let bundles = self.generate_bundles.unwrap()(self.resource_ids.clone());
    //     let mut i = Box::pin(bundles);
    //     while let Some(bundle) = (i.next()).await {
    //         if let Some(msg) = bundle.get_message(id) {
    //             if let Some(pattern) = msg.value {
    //                 let mut errors = vec![];
    //                 let val: Cow<'_, str> = bundle.format_pattern(pattern, args, &mut errors);
    //                 return val.to_string().into();
    //             }
    //         }
    //     }
    //     "Missing".into()
    // }
    //
    // pub fn format_value_sync(&mut self, id: &'l str, args: Option<&'l FluentArgs>) -> Cow<'l, str>
    // where
    //     R: Borrow<FluentResource>,
    // {
    //     let bundles = self.generate_bundles_sync.unwrap()(self.resource_ids.clone());
    //     for bundle in bundles {
    //         if let Some(msg) = bundle.get_message(id) {
    //             if let Some(pattern) = msg.value {
    //                 let mut errors = vec![];
    //                 let val: Cow<'_, str> = bundle.format_pattern(pattern, args, &mut errors);
    //                 return val.to_string().into();
    //             }
    //         }
    //     }
    //     "Missing".into()
    // }
}
