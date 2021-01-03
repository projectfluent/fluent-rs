use fluent_bundle::{FluentBundle, FluentError, FluentResource};
use futures::Stream;
use std::borrow::Borrow;

pub type FluentBundleResult<R> = Result<FluentBundle<R>, (FluentBundle<R>, Vec<FluentError>)>;

pub trait BundleIterator:
    Iterator<Item = FluentBundleResult<<Self as BundleIterator>::Resource>>
{
    type Resource: Borrow<FluentResource>;

    fn prefetch(&mut self) {}
}

pub trait BundleStream:
    Stream<Item = FluentBundleResult<<Self as BundleStream>::Resource>>
{
    type Resource: Borrow<FluentResource>;

    fn prefetch(&mut self) {}
}

pub trait BundleGenerator {
    type Resource: Borrow<FluentResource>;
    type Iter: BundleIterator;
    type Stream: BundleStream;

    // Can we make it a slice?
    fn bundles_iter(&self, res_ids: Vec<String>) -> Self::Iter;
    fn bundles_stream(&self, res_ids: Vec<String>) -> Self::Stream;
}
