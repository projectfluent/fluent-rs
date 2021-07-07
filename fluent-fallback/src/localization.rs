use crate::cache::{AsyncCache, Cache};
use crate::env::LocalesProvider;
use crate::errors::LocalizationError;
use crate::generator::{BundleGenerator, BundleIterator, BundleStream};
use crate::types::{L10nAttribute, L10nKey, L10nMessage};
use fluent_bundle::{FluentArgs, FluentBundle, FluentError};
use once_cell::sync::OnceCell;
use std::borrow::Cow;
use std::rc::Rc;

pub enum Bundles<G>
where
    G: BundleGenerator,
{
    Iter(Cache<G::Iter, G::Resource>),
    Stream(AsyncCache<G::Stream, G::Resource>),
}

impl<G> Bundles<G>
where
    G: BundleGenerator,
    G::Iter: BundleIterator,
{
    fn prefetch_sync(&self) {
        match self {
            Self::Iter(iter) => iter.prefetch(),
            Self::Stream(_) => todo!(),
        }
    }
}

impl<G> Bundles<G>
where
    G: BundleGenerator,
    G::Stream: BundleStream,
{
    async fn prefetch_async(&self) {
        match self {
            Self::Iter(_) => {
                todo!();
            }
            Self::Stream(stream) => stream.prefetch().await,
        }
    }
}

impl<G> Bundles<G>
where
    G: BundleGenerator,
{
    pub async fn format_value<'l>(
        &'l self,
        id: &'l str,
        args: Option<&'l FluentArgs<'_>>,
        errors: &mut Vec<LocalizationError>,
    ) -> Option<Cow<'l, str>> {
        match self {
            Bundles::Iter(cache) => Self::format_value_from_iter(cache, id, args, errors),
            Bundles::Stream(stream) => {
                Self::format_value_from_stream(stream, id, args, errors).await
            }
        }
    }

    pub async fn format_values<'l>(
        &'l self,
        keys: &'l [L10nKey<'l>],
        errors: &mut Vec<LocalizationError>,
    ) -> Vec<Option<Cow<'l, str>>> {
        match self {
            Bundles::Iter(cache) => Self::format_values_from_iter(cache, keys, errors),
            Bundles::Stream(stream) => Self::format_values_from_stream(stream, keys, errors).await,
        }
    }

    pub async fn format_messages<'l>(
        &'l self,
        keys: &'l [L10nKey<'l>],
        errors: &mut Vec<LocalizationError>,
    ) -> Vec<Option<L10nMessage<'l>>> {
        match self {
            Bundles::Iter(cache) => Self::format_messages_from_iter(cache, keys, errors),
            Bundles::Stream(stream) => {
                Self::format_messages_from_stream(stream, keys, errors).await
            }
        }
    }

    pub fn format_value_sync<'l>(
        &'l self,
        id: &'l str,
        args: Option<&'l FluentArgs>,
        errors: &mut Vec<LocalizationError>,
    ) -> Result<Option<Cow<'l, str>>, LocalizationError> {
        match self {
            Bundles::Iter(cache) => Ok(Self::format_value_from_iter(cache, id, args, errors)),
            Bundles::Stream(_) => Err(LocalizationError::SyncRequestInAsyncMode),
        }
    }

    pub fn format_values_sync<'l>(
        &'l self,
        keys: &'l [L10nKey<'l>],
        errors: &mut Vec<LocalizationError>,
    ) -> Result<Vec<Option<Cow<'l, str>>>, LocalizationError> {
        match self {
            Bundles::Iter(cache) => Ok(Self::format_values_from_iter(cache, keys, errors)),
            Bundles::Stream(_) => Err(LocalizationError::SyncRequestInAsyncMode),
        }
    }

    pub fn format_messages_sync<'l>(
        &'l self,
        keys: &'l [L10nKey<'l>],
        errors: &mut Vec<LocalizationError>,
    ) -> Result<Vec<Option<L10nMessage<'l>>>, LocalizationError> {
        match self {
            Bundles::Iter(cache) => Ok(Self::format_messages_from_iter(cache, keys, errors)),
            Bundles::Stream(_) => Err(LocalizationError::SyncRequestInAsyncMode),
        }
    }

    fn format_value_from_iter<'l>(
        cache: &'l Cache<G::Iter, G::Resource>,
        id: &'l str,
        args: Option<&'l FluentArgs>,
        errors: &mut Vec<LocalizationError>,
    ) -> Option<Cow<'l, str>> {
        let mut found_message = false;

        for bundle in cache {
            let bundle = bundle.as_ref().unwrap_or_else(|(bundle, err)| {
                errors.extend(err.iter().cloned().map(Into::into));
                bundle
            });

            if let Some(msg) = bundle.get_message(id) {
                found_message = true;
                if let Some(value) = msg.value() {
                    let mut format_errors = vec![];
                    let result = bundle.format_pattern(value, args, &mut format_errors);
                    if !format_errors.is_empty() {
                        errors.push(LocalizationError::Resolver {
                            id: id.to_string(),
                            locale: bundle.locales.get(0).cloned().unwrap(),
                            errors: format_errors,
                        });
                    }
                    return Some(result);
                } else {
                    errors.push(LocalizationError::MissingValue {
                        id: id.to_string(),
                        locale: Some(bundle.locales[0].clone()),
                    });
                }
            } else {
                errors.push(LocalizationError::MissingMessage {
                    id: id.to_string(),
                    locale: Some(bundle.locales[0].clone()),
                });
            }
        }
        if found_message {
            errors.push(LocalizationError::MissingValue {
                id: id.to_string(),
                locale: None,
            });
        } else {
            errors.push(LocalizationError::MissingMessage {
                id: id.to_string(),
                locale: None,
            });
        }
        None
    }

    async fn format_value_from_stream<'l>(
        stream: &'l AsyncCache<G::Stream, G::Resource>,
        id: &'l str,
        args: Option<&'l FluentArgs<'_>>,
        errors: &mut Vec<LocalizationError>,
    ) -> Option<Cow<'l, str>> {
        use futures::StreamExt;
        let mut found_message = false;

        let mut bundle_stream = stream.stream();
        while let Some(bundle) = bundle_stream.next().await {
            let bundle = bundle.as_ref().unwrap_or_else(|(bundle, err)| {
                errors.extend(err.iter().cloned().map(Into::into));
                bundle
            });

            if let Some(msg) = bundle.get_message(id) {
                found_message = true;
                if let Some(value) = msg.value() {
                    let mut format_errors = vec![];
                    let result = bundle.format_pattern(value, args, &mut format_errors);
                    if !format_errors.is_empty() {
                        errors.push(LocalizationError::Resolver {
                            id: id.to_string(),
                            locale: bundle.locales.get(0).cloned().unwrap(),
                            errors: format_errors,
                        });
                    }
                    return Some(result);
                } else {
                    errors.push(LocalizationError::MissingValue {
                        id: id.to_string(),
                        locale: Some(bundle.locales[0].clone()),
                    });
                }
            } else {
                errors.push(LocalizationError::MissingMessage {
                    id: id.to_string(),
                    locale: Some(bundle.locales[0].clone()),
                });
            }
        }
        if found_message {
            errors.push(LocalizationError::MissingValue {
                id: id.to_string(),
                locale: None,
            });
        } else {
            errors.push(LocalizationError::MissingMessage {
                id: id.to_string(),
                locale: None,
            });
        }
        None
    }

    async fn format_messages_from_stream<'l>(
        stream: &'l AsyncCache<G::Stream, G::Resource>,
        keys: &'l [L10nKey<'l>],
        errors: &mut Vec<LocalizationError>,
    ) -> Vec<Option<L10nMessage<'l>>> {
        let mut result: Vec<Option<L10nMessage>> = Vec::with_capacity(keys.len());

        for _ in 0..keys.len() {
            result.push(None);
        }

        let mut is_complete = false;

        use futures::StreamExt;
        let mut bundle_stream = stream.stream();
        while let Some(bundle) = bundle_stream.next().await {
            let bundle = bundle.as_ref().unwrap_or_else(|(bundle, err)| {
                errors.extend(err.iter().cloned().map(Into::into));
                bundle
            });

            let mut has_missing = false;
            for (key, cell) in keys
                .iter()
                .zip(&mut result)
                .filter(|(_, cell)| cell.is_none())
            {
                let mut format_errors = vec![];

                let msg = Self::format_message_from_bundle(bundle, key, &mut format_errors);

                if msg.is_none() {
                    has_missing = true;
                    errors.push(LocalizationError::MissingMessage {
                        id: key.id.to_string(),
                        locale: Some(bundle.locales[0].clone()),
                    });
                } else if !format_errors.is_empty() {
                    errors.push(LocalizationError::Resolver {
                        id: key.id.to_string(),
                        locale: bundle.locales.get(0).cloned().unwrap(),
                        errors: format_errors,
                    });
                }

                *cell = msg;
            }
            if !has_missing {
                is_complete = true;
                break;
            }
        }

        if !is_complete {
            for (key, _) in keys
                .iter()
                .zip(&mut result)
                .filter(|(_, cell)| cell.is_none())
            {
                errors.push(LocalizationError::MissingMessage {
                    id: key.id.to_string(),
                    locale: None,
                });
            }
        }

        result
    }

    async fn format_values_from_stream<'l>(
        stream: &'l AsyncCache<G::Stream, G::Resource>,
        keys: &'l [L10nKey<'l>],
        errors: &mut Vec<LocalizationError>,
    ) -> Vec<Option<Cow<'l, str>>> {
        enum Value<'l> {
            Value(Cow<'l, str>),
            MissingValue,
            None,
        }

        let mut cells: Vec<Value> = Vec::with_capacity(keys.len());

        for _ in 0..keys.len() {
            cells.push(Value::None);
        }

        use futures::StreamExt;
        let mut bundle_stream = stream.stream();
        while let Some(bundle) = bundle_stream.next().await {
            let bundle = bundle.as_ref().unwrap_or_else(|(bundle, err)| {
                errors.extend(err.iter().cloned().map(Into::into));
                bundle
            });

            let mut has_missing = false;

            for (key, cell) in keys
                .iter()
                .zip(&mut cells)
                .filter(|(_, cell)| !matches!(cell, Value::Value(_)))
            {
                if let Some(msg) = bundle.get_message(&key.id) {
                    if let Some(value) = msg.value() {
                        let mut format_errors = vec![];
                        *cell = Value::Value(bundle.format_pattern(
                            value,
                            key.args.as_ref(),
                            &mut format_errors,
                        ));
                        if !format_errors.is_empty() {
                            errors.push(LocalizationError::Resolver {
                                id: key.id.to_string(),
                                locale: bundle.locales.get(0).cloned().unwrap(),
                                errors: format_errors,
                            });
                        }
                    } else {
                        *cell = Value::MissingValue;
                        has_missing = true;
                        errors.push(LocalizationError::MissingValue {
                            id: key.id.to_string(),
                            locale: Some(bundle.locales[0].clone()),
                        });
                    }
                } else {
                    has_missing = true;
                    errors.push(LocalizationError::MissingMessage {
                        id: key.id.to_string(),
                        locale: Some(bundle.locales[0].clone()),
                    });
                }
            }
            if !has_missing {
                break;
            }
        }

        keys.iter()
            .zip(cells)
            .map(|(key, value)| match value {
                Value::Value(value) => Some(value),
                Value::MissingValue => {
                    errors.push(LocalizationError::MissingValue {
                        id: key.id.to_string(),
                        locale: None,
                    });
                    None
                }
                Value::None => {
                    errors.push(LocalizationError::MissingMessage {
                        id: key.id.to_string(),
                        locale: None,
                    });
                    None
                }
            })
            .collect()
    }

    fn format_message_from_bundle<'l>(
        bundle: &'l FluentBundle<G::Resource>,
        key: &'l L10nKey,
        format_errors: &mut Vec<FluentError>,
    ) -> Option<L10nMessage<'l>> {
        if let Some(msg) = bundle.get_message(&key.id) {
            let value = msg
                .value()
                .map(|pattern| bundle.format_pattern(pattern, key.args.as_ref(), format_errors));
            let attributes = msg
                .attributes()
                .map(|attr| {
                    let value =
                        bundle.format_pattern(attr.value(), key.args.as_ref(), format_errors);
                    L10nAttribute {
                        name: attr.id().into(),
                        value,
                    }
                })
                .collect();
            Some(L10nMessage { value, attributes })
        } else {
            None
        }
    }

    fn format_messages_from_iter<'l>(
        cache: &'l Cache<G::Iter, G::Resource>,
        keys: &'l [L10nKey<'l>],
        errors: &mut Vec<LocalizationError>,
    ) -> Vec<Option<L10nMessage<'l>>> {
        let mut result: Vec<Option<L10nMessage>> = Vec::with_capacity(keys.len());

        for _ in 0..keys.len() {
            result.push(None);
        }

        let mut is_complete = false;

        for bundle in cache {
            let bundle = bundle.as_ref().unwrap_or_else(|(bundle, err)| {
                errors.extend(err.iter().cloned().map(Into::into));
                bundle
            });

            let mut has_missing = false;
            for (key, cell) in keys
                .iter()
                .zip(&mut result)
                .filter(|(_, cell)| cell.is_none())
            {
                let mut format_errors = vec![];
                let msg = Self::format_message_from_bundle(bundle, key, &mut format_errors);

                if msg.is_none() {
                    has_missing = true;
                    errors.push(LocalizationError::MissingMessage {
                        id: key.id.to_string(),
                        locale: Some(bundle.locales[0].clone()),
                    });
                } else if !format_errors.is_empty() {
                    errors.push(LocalizationError::Resolver {
                        id: key.id.to_string(),
                        locale: bundle.locales.get(0).cloned().unwrap(),
                        errors: format_errors,
                    });
                }

                *cell = msg;
            }
            if !has_missing {
                is_complete = true;
                break;
            }
        }

        if !is_complete {
            for (key, _) in keys
                .iter()
                .zip(&mut result)
                .filter(|(_, cell)| cell.is_none())
            {
                errors.push(LocalizationError::MissingMessage {
                    id: key.id.to_string(),
                    locale: None,
                });
            }
        }

        result
    }

    fn format_values_from_iter<'l>(
        cache: &'l Cache<G::Iter, G::Resource>,
        keys: &'l [L10nKey<'l>],
        errors: &mut Vec<LocalizationError>,
    ) -> Vec<Option<Cow<'l, str>>> {
        enum Value<'l> {
            Value(Cow<'l, str>),
            MissingValue,
            None,
        }

        let mut cells: Vec<Value> = Vec::with_capacity(keys.len());

        for _ in 0..keys.len() {
            cells.push(Value::None);
        }

        for bundle in cache {
            let bundle = bundle.as_ref().unwrap_or_else(|(bundle, err)| {
                errors.extend(err.iter().cloned().map(Into::into));
                bundle
            });

            let mut has_missing = false;

            for (key, cell) in keys
                .iter()
                .zip(&mut cells)
                .filter(|(_, cell)| !matches!(cell, Value::Value(_)))
            {
                if let Some(msg) = bundle.get_message(&key.id) {
                    if let Some(value) = msg.value() {
                        let mut format_errors = vec![];
                        *cell = Value::Value(bundle.format_pattern(
                            value,
                            key.args.as_ref(),
                            &mut format_errors,
                        ));
                        if !format_errors.is_empty() {
                            errors.push(LocalizationError::Resolver {
                                id: key.id.to_string(),
                                locale: bundle.locales.get(0).cloned().unwrap(),
                                errors: format_errors,
                            });
                        }
                    } else {
                        *cell = Value::MissingValue;
                        has_missing = true;
                        errors.push(LocalizationError::MissingValue {
                            id: key.id.to_string(),
                            locale: Some(bundle.locales[0].clone()),
                        });
                    }
                } else {
                    has_missing = true;
                    errors.push(LocalizationError::MissingMessage {
                        id: key.id.to_string(),
                        locale: Some(bundle.locales[0].clone()),
                    });
                }
            }
            if !has_missing {
                break;
            }
        }

        keys.iter()
            .zip(cells)
            .map(|(key, value)| match value {
                Value::Value(value) => Some(value),
                Value::MissingValue => {
                    errors.push(LocalizationError::MissingValue {
                        id: key.id.to_string(),
                        locale: None,
                    });
                    None
                }
                Value::None => {
                    errors.push(LocalizationError::MissingMessage {
                        id: key.id.to_string(),
                        locale: None,
                    });
                    None
                }
            })
            .collect()
    }
}

pub struct Localization<G, P>
where
    G: BundleGenerator<LocalesIter = P::Iter>,
    P: LocalesProvider,
{
    bundles: OnceCell<Rc<Bundles<G>>>,
    generator: G,
    provider: P,
    sync: bool,
    res_ids: Vec<String>,
}

impl<G, P> Localization<G, P>
where
    G: BundleGenerator<LocalesIter = P::Iter> + Default,
    P: LocalesProvider + Default,
{
    pub fn new(res_ids: Vec<String>, sync: bool) -> Self {
        Self {
            bundles: OnceCell::new(),
            generator: G::default(),
            provider: P::default(),
            sync,
            res_ids,
        }
    }
}

impl<G, P> Localization<G, P>
where
    G: BundleGenerator<LocalesIter = P::Iter>,
    P: LocalesProvider,
{
    pub fn with_env(res_ids: Vec<String>, sync: bool, provider: P, generator: G) -> Self {
        Self {
            bundles: OnceCell::new(),
            generator,
            provider,
            sync,
            res_ids,
        }
    }

    pub fn is_sync(&self) -> bool {
        self.sync
    }

    pub fn add_resource_id(&mut self, res_id: String) {
        self.res_ids.push(res_id);
        self.on_change();
    }

    pub fn add_resource_ids(&mut self, res_ids: Vec<String>) {
        self.res_ids.extend(res_ids);
        self.on_change();
    }

    pub fn remove_resource_id(&mut self, res_id: String) -> usize {
        self.res_ids.retain(|x| *x != res_id);
        self.on_change();
        self.res_ids.len()
    }

    pub fn remove_resource_ids(&mut self, res_ids: Vec<String>) -> usize {
        self.res_ids.retain(|x| !res_ids.contains(x));
        self.on_change();
        self.res_ids.len()
    }

    pub fn set_async(&mut self) {
        if self.sync {
            self.sync = false;
            self.on_change();
        }
    }

    pub fn on_change(&mut self) {
        self.bundles.take();
    }
}

impl<G, P> Localization<G, P>
where
    G: BundleGenerator<LocalesIter = P::Iter>,
    G::Iter: BundleIterator,
    P: LocalesProvider,
{
    pub fn prefetch_sync(&mut self) {
        let bundles = self.bundles();
        bundles.prefetch_sync();
    }
}

impl<G, P> Localization<G, P>
where
    G: BundleGenerator<LocalesIter = P::Iter>,
    G::Stream: BundleStream,
    P: LocalesProvider,
{
    pub async fn prefetch_async(&mut self) {
        let bundles = self.bundles();
        bundles.prefetch_async().await
    }
}

impl<G, P> Localization<G, P>
where
    G: BundleGenerator<LocalesIter = P::Iter>,
    P: LocalesProvider,
{
    pub fn bundles(&self) -> Rc<Bundles<G>> {
        self.bundles
            .get_or_init(|| {
                if self.sync {
                    Rc::new(Bundles::Iter(Cache::new(
                        self.generator
                            .bundles_iter(self.provider.locales(), self.res_ids.clone()),
                    )))
                } else {
                    Rc::new(Bundles::Stream(AsyncCache::new(
                        self.generator
                            .bundles_stream(self.provider.locales(), self.res_ids.clone()),
                    )))
                }
            })
            .clone()
    }
}
