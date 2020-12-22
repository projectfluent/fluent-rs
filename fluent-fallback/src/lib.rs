use cache::{AsyncCache, Cache};
use fluent_bundle::{FluentArgs, FluentBundle, FluentError, FluentResource};
use futures::Stream;

use std::{
    borrow::{Borrow, Cow},
    ops::{Deref, DerefMut},
};

mod cache;

pub trait BundleGeneratorSync {
    type Resource;
    type Iter: Iterator<Item = FluentBundle<Self::Resource>>;

    fn bundles_sync(&self, resource_ids: Vec<String>) -> Self::Iter;
}

pub trait BundleGenerator {
    type Resource;
    type Stream: Stream<Item = FluentBundle<Self::Resource>>;

    fn bundles(&self, resource_ids: Vec<String>) -> Self::Stream;
}

pub struct L10nKey<'l> {
    pub id: String,
    pub args: Option<FluentArgs<'l>>,
}

#[derive(Debug)]
pub struct L10nAttribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct L10nMessage {
    pub value: Option<String>,
    pub attributes: Vec<L10nAttribute>,
}

pub struct SyncLocalization<G: BundleGeneratorSync> {
    resource_ids: Vec<String>,
    bundles: Cache<G::Iter>,
    generator: G,
}

impl<G> SyncLocalization<G>
where
    G: BundleGeneratorSync + Default,
{
    pub fn new(resource_ids: Vec<String>) -> Self {
        Self::with_generator(resource_ids, G::default())
    }
}

impl<G> SyncLocalization<G>
where
    G: BundleGeneratorSync,
{
    pub fn with_generator(resource_ids: Vec<String>, generator: G) -> Self {
        let bundles = Cache::new(generator.bundles_sync(resource_ids.clone()));

        Self {
            resource_ids,
            bundles,
            generator,
        }
    }

    pub fn add_resource_ids(&mut self, res_ids: Vec<String>) -> usize {
        for res_id in res_ids {
            if !self.resource_ids.contains(&res_id) {
                self.resource_ids.push(res_id);
            }
        }
        self.on_change();
        self.resource_ids.len()
    }

    pub fn remove_resource_ids(&mut self, res_ids: Vec<String>) -> usize {
        self.resource_ids.retain(|id| !res_ids.contains(id));
        self.on_change();
        self.resource_ids.len()
    }

    pub fn is_sync(&self) -> bool {
        true
    }

    pub fn on_change(&mut self) {
        // This invalidates the cache by recreating it.
        self.bundles = Cache::new(self.generator.bundles_sync(self.resource_ids.clone()));
    }

    pub fn prefetch_sync(&self) {
        self.bundles.into_iter().next();
    }

    fn format_value_sync_opt<'l>(
        &'l self,
        id: &str,
        args: Option<&'l FluentArgs>,
        errors: &mut Vec<FluentError>,
    ) -> Option<Cow<'_, str>>
    where
        G::Resource: Borrow<FluentResource>,
    {
        for bundle in &self.bundles {
            if let Some(msg) = bundle.get_message(id) {
                if let Some(pattern) = msg.value {
                    return Some(bundle.format_pattern(pattern, args, errors));
                }
            }
        }
        None
    }

    fn format_message_sync_opt<'l>(
        &'l self,
        id: &str,
        args: Option<&'l FluentArgs>,
        errors: &mut Vec<FluentError>,
    ) -> Option<L10nMessage>
    where
        G::Resource: Borrow<FluentResource>,
    {
        for bundle in &self.bundles {
            if let Some(msg) = bundle.get_message(id) {
                let value = msg
                    .value
                    .map(|pattern| bundle.format_pattern(pattern, args, errors).into_owned());
                let attributes = msg
                    .attributes
                    .iter()
                    .map(|attr| {
                        let value = bundle.format_pattern(attr.value, args, errors).into_owned();
                        L10nAttribute {
                            name: attr.id.to_string(),
                            value,
                        }
                    })
                    .collect();
                return Some(L10nMessage { value, attributes });
            }
        }
        None
    }

    pub fn format_value_sync<'l>(
        &'l self,
        id: &'l str,
        args: Option<&'l FluentArgs>,
    ) -> Cow<'l, str>
    where
        G::Resource: Borrow<FluentResource>,
    {
        let mut _errors = vec![];

        self.format_value_sync_opt(id, args, &mut _errors)
            .unwrap_or_else(|| id.into())
    }

    pub fn format_values_sync<'l>(&'l self, keys: &'l [L10nKey<'l>]) -> Vec<Option<Cow<'l, str>>>
    where
        G::Resource: Borrow<FluentResource>,
    {
        let mut errors = vec![];

        keys.iter()
            .map(|key| self.format_value_sync_opt(&key.id, key.args.as_ref(), &mut errors))
            .collect::<Vec<_>>()
    }

    pub fn format_messages_sync<'l>(&'l self, keys: &'l [L10nKey<'l>]) -> Vec<Option<L10nMessage>>
    where
        G::Resource: Borrow<FluentResource>,
    {
        let mut errors = vec![];
        keys.iter()
            .map(|key| self.format_message_sync_opt(&key.id, key.args.as_ref(), &mut errors))
            .collect::<Vec<_>>()
    }
}

impl<G> SyncLocalization<G>
where
    G: BundleGenerator + BundleGeneratorSync,
{
    pub fn upgrade(self) -> AsyncLocalization<G> {
        let Self {
            resource_ids,
            generator,
            ..
        } = self;

        let bundles = AsyncCache::new(generator.bundles(resource_ids.clone()));
        AsyncLocalization {
            resource_ids,
            bundles,
            generator,
        }
    }
}

impl<G> Deref for SyncLocalization<G>
where
    G: BundleGeneratorSync,
{
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.generator
    }
}

impl<G> DerefMut for SyncLocalization<G>
where
    G: BundleGeneratorSync,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.generator
    }
}

pub struct AsyncLocalization<G: BundleGenerator> {
    resource_ids: Vec<String>,
    bundles: AsyncCache<G::Stream>,
    generator: G,
}

impl<G> AsyncLocalization<G>
where
    G: BundleGenerator + Default,
{
    pub fn new(resource_ids: Vec<String>) -> Self {
        Self::with_generator(resource_ids, G::default())
    }
}

impl<G> AsyncLocalization<G>
where
    G: BundleGenerator,
{
    pub fn with_generator(resource_ids: Vec<String>, generator: G) -> Self {
        let bundles = AsyncCache::new(generator.bundles(resource_ids.clone()));

        Self {
            resource_ids,
            bundles,
            generator,
        }
    }

    pub fn add_resource_ids(&mut self, res_ids: Vec<String>) -> usize {
        for res_id in res_ids {
            if !self.resource_ids.contains(&res_id) {
                self.resource_ids.push(res_id);
            }
        }
        self.on_change();
        self.resource_ids.len()
    }

    pub fn remove_resource_ids(&mut self, res_ids: Vec<String>) -> usize {
        self.resource_ids.retain(|id| !res_ids.contains(id));
        self.on_change();
        self.resource_ids.len()
    }

    pub async fn prefetch(&self) {
        use futures::StreamExt;
        let mut bundle_stream = self.bundles.stream();
        bundle_stream.next().await;
    }

    pub fn is_sync(&self) -> bool {
        false
    }

    pub fn on_change(&mut self) {
        // This invalidates the cache by recreating it.
        self.bundles = AsyncCache::new(self.generator.bundles(self.resource_ids.clone()));
    }

    async fn format_value_opt<'l>(
        &'l self,
        id: &str,
        args: Option<&'l FluentArgs<'l>>,
        errors: &mut Vec<FluentError>,
    ) -> Option<Cow<'_, str>>
    where
        G::Resource: Borrow<FluentResource>,
    {
        use futures::StreamExt;
        let mut bundle_stream = self.bundles.stream();
        while let Some(bundle) = bundle_stream.next().await {
            if let Some(msg) = bundle.get_message(id) {
                if let Some(pattern) = msg.value {
                    return Some(bundle.format_pattern(pattern, args, errors));
                }
            }
        }
        None
    }

    async fn format_message_opt<'l>(
        &'l self,
        id: &str,
        args: Option<&'l FluentArgs<'l>>,
        errors: &mut Vec<FluentError>,
    ) -> Option<L10nMessage>
    where
        G::Resource: Borrow<FluentResource>,
    {
        use futures::StreamExt;
        let mut bundle_stream = self.bundles.stream();
        while let Some(bundle) = bundle_stream.next().await {
            if let Some(msg) = bundle.get_message(id) {
                let value = msg
                    .value
                    .map(|pattern| bundle.format_pattern(pattern, args, errors).into_owned());
                let attributes = msg
                    .attributes
                    .iter()
                    .map(|attr| {
                        let value = bundle.format_pattern(attr.value, args, errors).into_owned();
                        L10nAttribute {
                            name: attr.id.to_string(),
                            value,
                        }
                    })
                    .collect();
                return Some(L10nMessage { value, attributes });
            }
        }
        None
    }

    pub async fn format_value<'l>(
        &'l self,
        id: &'l str,
        args: Option<&'l FluentArgs<'l>>,
    ) -> Cow<'l, str>
    where
        G::Resource: Borrow<FluentResource>,
    {
        let mut _errors = vec![];

        self.format_value_opt(id, args, &mut _errors)
            .await
            .unwrap_or_else(|| id.into())
    }

    pub async fn format_values<'l>(&'l self, keys: &'l [L10nKey<'l>]) -> Vec<Option<Cow<'l, str>>>
    where
        G::Resource: Borrow<FluentResource>,
    {
        let mut errors = vec![];
        let mut results = vec![];
        let mut i = 0;
        while i < keys.len() {
            let key = &keys[i];
            let value = self
                .format_value_opt(&key.id, key.args.as_ref(), &mut errors)
                .await;
            results.push(value);
            i += 1;
        }
        results
    }

    pub async fn format_messages<'l>(&'l self, keys: &'l [L10nKey<'l>]) -> Vec<Option<L10nMessage>>
    where
        G::Resource: Borrow<FluentResource>,
    {
        let mut errors = vec![];
        let mut results = vec![];
        let mut i = 0;
        while i < keys.len() {
            let key = &keys[i];
            let value = self
                .format_message_opt(&key.id, key.args.as_ref(), &mut errors)
                .await;
            results.push(value);
            i += 1;
        }
        results
    }
}

impl<G> Deref for AsyncLocalization<G>
where
    G: BundleGenerator,
{
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.generator
    }
}

impl<G> DerefMut for AsyncLocalization<G>
where
    G: BundleGenerator,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.generator
    }
}
