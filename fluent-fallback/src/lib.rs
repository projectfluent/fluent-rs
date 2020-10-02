use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use std::borrow::Borrow;
use std::borrow::Cow;

use reiterate::Reiterate;

pub type BundleIterator<R> = dyn Iterator<Item = Box<FluentBundle<R>>>;

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

pub struct Localization<R> {
    pub resource_ids: Vec<String>,
    bundles: Reiterate<Box<BundleIterator<R>>>,
    generate_bundles_sync: Box<dyn FnMut(Vec<String>) -> Box<BundleIterator<R>>>,
}

impl<R> Localization<R> {
    pub fn new<F: 'static>(resource_ids: Vec<String>, mut generate_bundles_sync: F) -> Self
    where
        F: FnMut(Vec<String>) -> Box<BundleIterator<R>>,
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

    pub fn format_values_sync<'l>(&'l self, keys: &'l [L10nKey<'l>]) -> Vec<Option<Cow<'l, str>>>
    where
        R: Borrow<FluentResource>,
    {
        let mut errors = vec![];
        let mut result: Vec<Option<Cow<'l, str>>> = vec![];
        result.resize_with(keys.len(), Default::default);

        for (i, key) in keys.iter().enumerate() {
            for bundle in &self.bundles {
                if let Some(msg) = bundle.get_message(&key.id) {
                    if let Some(pattern) = msg.value {
                        let val = bundle.format_pattern(pattern, key.args.as_ref(), &mut errors);
                        result[i] = Some(val.clone());
                        break;
                    }
                }
            }
        }
        result
    }

    pub fn format_messages_sync<'l>(&'l self, keys: &'l [L10nKey<'l>]) -> Vec<Option<L10nMessage>>
    where
        R: Borrow<FluentResource>,
    {
        let mut errors = vec![];
        let mut result: Vec<Option<L10nMessage>> = vec![];
        result.resize_with(keys.len(), Default::default);

        for (i, key) in keys.iter().enumerate() {
            for bundle in &self.bundles {
                if let Some(msg) = bundle.get_message(&key.id) {
                    let value = msg.value.map(|pattern| {
                        bundle
                            .format_pattern(pattern, key.args.as_ref(), &mut errors)
                            .into_owned()
                    });
                    let attributes = msg
                        .attributes
                        .iter()
                        .map(|attr| {
                            let value = bundle
                                .format_pattern(attr.value, key.args.as_ref(), &mut errors)
                                .into_owned();
                            L10nAttribute {
                                name: attr.id.to_string(),
                                value,
                            }
                        })
                        .collect();
                    result[i] = Some(L10nMessage { value, attributes });
                }
            }
        }
        result
    }
}
