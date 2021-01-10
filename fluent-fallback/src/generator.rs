use fluent_bundle::{FluentBundle, FluentError, FluentResource};
use futures::Stream;
use std::borrow::Borrow;

pub type FluentBundleResult<R> = Result<FluentBundle<R>, (FluentBundle<R>, Vec<FluentError>)>;

pub trait BundleIterator {
    fn prefetch_sync(&mut self) {}
}

#[async_trait::async_trait(?Send)]
pub trait BundleStream {
    async fn prefetch_async(&mut self) {}
}

pub trait BundleGenerator {
    type Resource: Borrow<FluentResource>;
    type Iter: Iterator<Item = FluentBundleResult<Self::Resource>>;
    type Stream: Stream<Item = FluentBundleResult<Self::Resource>>;

    // Can we make it a slice?
    fn bundles_iter(&self, res_ids: Vec<String>) -> Self::Iter;
    fn bundles_stream(&self, res_ids: Vec<String>) -> Self::Stream;
}
