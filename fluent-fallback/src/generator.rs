use fluent_bundle::{FluentBundle, FluentError, FluentResource};
use futures::Stream;
use std::borrow::Borrow;

pub type FluentBundleResult<R> = Result<FluentBundle<R>, (FluentBundle<R>, Vec<FluentError>)>;

pub trait BundleIterator<R>: Iterator<Item = FluentBundleResult<R>> {
    fn prefetch(&mut self) {}
}

pub trait BundleStream<R>: Stream<Item = FluentBundleResult<R>> {
    fn prefetch(&mut self) {}
}

pub trait BundleGenerator {
    type Resource: Borrow<FluentResource>;
    type Iter: BundleIterator<Self::Resource>;
    type Stream: BundleStream<Self::Resource>;

    // Can we make it a slice?
    fn bundles_iter(&self, res_ids: Vec<String>) -> Self::Iter;
    fn bundles_stream(&self, res_ids: Vec<String>) -> Self::Stream;
}
