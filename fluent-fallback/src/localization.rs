use crate::cache::{AsyncCache, Cache};
use crate::errors::LocalizationError;
use crate::generator::BundleGenerator;
use crate::types::{L10nAttribute, L10nKey, L10nMessage};
use fluent_bundle::{FluentArgs, FluentError};
use once_cell::sync::OnceCell;
use std::borrow::Cow;

enum Bundles<G>
where
    G: BundleGenerator,
{
    Iter(Cache<G::Iter>),
    Stream(AsyncCache<G::Stream>),
}

impl<G> Bundles<G>
where
    G: BundleGenerator,
{
    fn prefetch(&self) {
        match self {
            Self::Iter(iter) => iter.prefetch(),
            Self::Stream(stream) => stream.prefetch(),
        }
    }
}

pub struct Localization<G>
where
    G: BundleGenerator,
{
    // Replace with `OneCell` once it stabilizes
    // https://github.com/rust-lang/rust/issues/74465
    bundles: OnceCell<Bundles<G>>,
    generator: G,
    res_ids: Vec<String>,
    sync: bool,
}

impl<G> Localization<G>
where
    G: BundleGenerator + Default,
{
    pub fn new(res_ids: Vec<String>, sync: bool) -> Self {
        Self {
            bundles: OnceCell::new(),
            generator: G::default(),
            res_ids,
            sync,
        }
    }
}

impl<G> Localization<G>
where
    G: BundleGenerator,
{
    pub fn with_generator(res_ids: Vec<String>, sync: bool, generator: G) -> Self {
        Self {
            bundles: OnceCell::new(),
            generator,
            res_ids,
            sync,
        }
    }

    pub fn add_resource_id(&mut self, res_id: String) {
        self.res_ids.push(res_id);
    }

    pub fn add_resource_ids(&mut self, res_ids: Vec<String>) {
        self.res_ids.extend(res_ids);
    }

    pub fn remove_resource_id(&mut self, res_id: String) -> usize {
        self.res_ids.retain(|x| *x != res_id);
        self.res_ids.len()
    }

    pub fn remove_resource_ids(&mut self, res_ids: Vec<String>) -> usize {
        self.res_ids.retain(|x| !res_ids.contains(x));
        self.res_ids.len()
    }

    pub fn prefetch(&self) {
        let bundles = self.get_bundles();
        bundles.prefetch();
    }

    pub fn set_async(&mut self) {
        if self.sync {
            self.bundles.take();
            self.sync = false;
        }
    }

    pub fn on_change(&mut self) {
        self.bundles.take();
    }

    pub async fn format_value<'l>(
        &'l self,
        id: &'l str,
        args: Option<&'l FluentArgs<'_>>,
        errors: &mut Vec<LocalizationError>,
    ) -> Cow<'l, str> {
        let mut format_errors = vec![];
        let result = match self.get_bundles() {
            Bundles::Iter(cache) => {
                Self::format_with_fallback_sync(cache, id, args, errors, &mut format_errors)
            }
            Bundles::Stream(stream) => {
                Self::format_with_fallback(stream, id, args, errors, &mut format_errors).await
            }
        };
        if !format_errors.is_empty() {
            errors.extend(format_errors.into_iter().map(|e| (id, e).into()));
        }
        result
    }

    pub async fn format_values<'l>(
        &'l self,
        keys: &'l [L10nKey<'l>],
        errors: &mut Vec<LocalizationError>,
    ) -> Vec<Cow<'l, str>> {
        match self.get_bundles() {
            Bundles::Iter(cache) => keys
                .iter()
                .map(|key| {
                    let mut format_errors = vec![];
                    let result = Self::format_with_fallback_sync(
                        cache,
                        &key.id,
                        key.args.as_ref(),
                        errors,
                        &mut format_errors,
                    );
                    if !format_errors.is_empty() {
                        errors.extend(format_errors.into_iter().map(|e| (&key.id, e).into()));
                    }
                    result
                })
                .collect::<Vec<_>>(),
            Bundles::Stream(stream) => {
                let mut result = Vec::with_capacity(keys.len());
                for key in keys {
                    let mut format_errors = vec![];
                    result.push(
                        Self::format_with_fallback(
                            stream,
                            &key.id,
                            key.args.as_ref(),
                            errors,
                            &mut format_errors,
                        )
                        .await,
                    );
                    if !format_errors.is_empty() {
                        errors.extend(format_errors.into_iter().map(|e| (&key.id, e).into()));
                    }
                }
                result
            }
        }
    }

    pub async fn format_messages<'l>(
        &'l self,
        keys: &'l [L10nKey<'l>],
        errors: &mut Vec<LocalizationError>,
    ) -> Vec<Option<L10nMessage<'l>>> {
        match self.get_bundles() {
            Bundles::Iter(cache) => keys
                .iter()
                .map(|key| {
                    let mut format_errors = vec![];
                    let result = Self::format_message_with_fallback_sync(
                        cache,
                        &key.id,
                        key.args.as_ref(),
                        errors,
                        &mut format_errors,
                    );
                    if !format_errors.is_empty() {
                        errors.extend(format_errors.into_iter().map(|e| (&key.id, e).into()));
                    }
                    result
                })
                .collect::<Vec<_>>(),
            Bundles::Stream(stream) => {
                let mut result = Vec::with_capacity(keys.len());
                for key in keys {
                    let mut format_errors = vec![];
                    result.push(
                        Self::format_message_with_fallback(
                            stream,
                            &key.id,
                            key.args.as_ref(),
                            errors,
                            &mut format_errors,
                        )
                        .await,
                    );
                    if !format_errors.is_empty() {
                        errors.extend(format_errors.into_iter().map(|e| (&key.id, e).into()));
                    }
                }
                result
            }
        }
    }

    pub fn format_value_sync<'l>(
        &'l self,
        id: &'l str,
        args: Option<&'l FluentArgs>,
        errors: &mut Vec<LocalizationError>,
    ) -> Result<Cow<'l, str>, LocalizationError> {
        let mut format_errors = vec![];
        let result = match self.get_bundles() {
            Bundles::Iter(cache) => {
                Ok(Self::format_with_fallback_sync(cache, id, args, errors, &mut format_errors))
            }
            Bundles::Stream(_) => Err(LocalizationError::SyncRequestInAsyncMode),
        };
        if !format_errors.is_empty() {
            errors.extend(format_errors.into_iter().map(|e| (id, e).into()));
        }
        result
    }

    pub fn format_values_sync<'l>(
        &'l self,
        keys: &'l [L10nKey<'l>],
        errors: &mut Vec<LocalizationError>,
    ) -> Result<Vec<Cow<'l, str>>, LocalizationError> {
        match self.get_bundles() {
            Bundles::Iter(cache) => Ok(keys
                .iter()
                .map(|key| {
                    let mut format_errors = vec![];
                    let result = Self::format_with_fallback_sync(
                        cache,
                        &key.id,
                        key.args.as_ref(),
                        errors,
                        &mut format_errors,
                    );
                    if !format_errors.is_empty() {
                        errors.extend(format_errors.into_iter().map(|e| (&key.id, e).into()));
                    }
                    result
                })
                .collect::<Vec<_>>()),
            Bundles::Stream(_) => Err(LocalizationError::SyncRequestInAsyncMode),
        }
    }

    pub fn format_messages_sync<'l>(
        &'l self,
        keys: &'l [L10nKey<'l>],
        errors: &mut Vec<LocalizationError>,
    ) -> Result<Vec<Option<L10nMessage<'l>>>, LocalizationError> {
        match self.get_bundles() {
            Bundles::Iter(cache) => Ok(keys
                .iter()
                .map(|key| {
                    let mut format_errors = vec![];
                    let result = Self::format_message_with_fallback_sync(
                        cache,
                        &key.id,
                        key.args.as_ref(),
                        errors,
                        &mut format_errors,
                    );
                    if !format_errors.is_empty() {
                        errors.extend(format_errors.into_iter().map(|e| (&key.id, e).into()));
                    }
                    result
                })
                .collect::<Vec<_>>()),
            Bundles::Stream(_) => Err(LocalizationError::SyncRequestInAsyncMode),
        }
    }
}

impl<G> Localization<G>
where
    G: BundleGenerator,
{
    fn get_bundles(&self) -> &Bundles<G> {
        self.bundles.get_or_init(|| {
            if self.sync {
                Bundles::Iter(Cache::new(
                    self.generator.bundles_iter(self.res_ids.clone()),
                ))
            } else {
                Bundles::Stream(AsyncCache::new(
                    self.generator.bundles_stream(self.res_ids.clone()),
                ))
            }
        })
    }

    fn format_with_fallback_sync<'l>(
        cache: &'l Cache<G::Iter>,
        id: &'l str,
        args: Option<&'l FluentArgs>,
        errors: &mut Vec<LocalizationError>,
        format_errors: &mut Vec<FluentError>,
    ) -> Cow<'l, str>
    where
        G::Resource: 'l,
    {
        for bundle in cache {
            let bundle = match bundle {
                Ok(bundle) => bundle,
                Err((bundle, err)) => {
                    errors.extend(err.iter().cloned().map(Into::into));
                    bundle
                }
            };
            if let Some(msg) = bundle.get_message(id) {
                if let Some(pattern) = msg.value {
                    return bundle.format_pattern(pattern, args, format_errors);
                } else {
                    errors.push(LocalizationError::MissingValue { id: id.to_string() });
                    return id.into();
                }
            }
        }
        errors.push(LocalizationError::MissingMessage { id: id.to_string() });
        id.into()
    }

    fn format_message_with_fallback_sync<'l>(
        cache: &'l Cache<G::Iter>,
        id: &'l str,
        args: Option<&'l FluentArgs>,
        errors: &mut Vec<LocalizationError>,
        format_errors: &mut Vec<FluentError>,
    ) -> Option<L10nMessage<'l>>
    where
        G::Resource: 'l,
    {
        for bundle in cache {
            let bundle = match bundle {
                Ok(bundle) => bundle,
                Err((bundle, err)) => {
                    errors.extend(err.iter().cloned().map(Into::into));
                    bundle
                }
            };
            if let Some(msg) = bundle.get_message(id) {
                let value = msg
                    .value
                    .map(|pattern| bundle.format_pattern(pattern, args, format_errors));
                let attributes = msg
                    .attributes
                    .iter()
                    .map(|attr| {
                        let value = bundle.format_pattern(attr.value, args, format_errors);
                        L10nAttribute {
                            name: attr.id.into(),
                            value,
                        }
                    })
                    .collect();
                return Some(L10nMessage { value, attributes });
            }
        }
        errors.push(LocalizationError::MissingMessage { id: id.to_string() });
        None
    }

    async fn format_message_with_fallback<'l>(
        stream: &'l AsyncCache<G::Stream>,
        id: &'l str,
        args: Option<&'l FluentArgs<'l>>,
        errors: &mut Vec<LocalizationError>,
        format_errors: &mut Vec<FluentError>,
    ) -> Option<L10nMessage<'l>>
    where
        G::Resource: 'l,
    {
        use futures::StreamExt;
        let mut bundle_stream = stream.stream();
        while let Some(bundle) = bundle_stream.next().await {
            let bundle = match bundle {
                Ok(bundle) => bundle,
                Err((bundle, err)) => {
                    errors.extend(err.iter().cloned().map(Into::into));
                    bundle
                }
            };
            if let Some(msg) = bundle.get_message(id) {
                let value = msg
                    .value
                    .map(|pattern| bundle.format_pattern(pattern, args, format_errors));
                let attributes = msg
                    .attributes
                    .iter()
                    .map(|attr| {
                        let value = bundle.format_pattern(attr.value, args, format_errors);
                        L10nAttribute {
                            name: attr.id.into(),
                            value,
                        }
                    })
                    .collect();
                return Some(L10nMessage { value, attributes });
            }
        }
        errors.push(LocalizationError::MissingMessage { id: id.to_string() });
        None
    }

    async fn format_with_fallback<'l>(
        stream: &'l AsyncCache<G::Stream>,
        id: &'l str,
        args: Option<&'l FluentArgs<'_>>,
        errors: &mut Vec<LocalizationError>,
        format_errors: &mut Vec<FluentError>,
    ) -> Cow<'l, str>
    where
        G::Resource: 'l,
    {
        use futures::StreamExt;
        let mut bundle_stream = stream.stream();
        while let Some(bundle) = bundle_stream.next().await {
            let bundle = match bundle {
                Ok(bundle) => bundle,
                Err((bundle, err)) => {
                    errors.extend(err.iter().cloned().map(Into::into));
                    bundle
                }
            };
            if let Some(msg) = bundle.get_message(id) {
                if let Some(pattern) = msg.value {
                    return bundle.format_pattern(pattern, args, format_errors);
                } else {
                    errors.push(LocalizationError::MissingValue { id: id.to_string() });
                    return id.into();
                }
            }
        }
        errors.push(LocalizationError::MissingMessage { id: id.to_string() });
        id.into()
    }
}
