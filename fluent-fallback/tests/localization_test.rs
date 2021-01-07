use std::borrow::Cow;
use std::fs;

use fluent_bundle::{FluentBundle, FluentResource};
use fluent_fallback::{
    generator::{BundleGenerator, BundleIterator, BundleStream, FluentBundleResult},
    Localization,
};
use std::cell::RefCell;
use std::rc::Rc;
use unic_langid::{langid, LanguageIdentifier};

struct InnerLocales {
    locales: RefCell<Vec<LanguageIdentifier>>,
}

impl InnerLocales {
    pub fn insert(&self, index: usize, element: LanguageIdentifier) {
        self.locales.borrow_mut().insert(index, element);
    }
}

#[derive(Clone)]
struct Locales {
    inner: Rc<InnerLocales>,
}

impl Locales {
    pub fn new(locales: Vec<LanguageIdentifier>) -> Self {
        Self {
            inner: Rc::new(InnerLocales {
                locales: RefCell::new(locales),
            }),
        }
    }

    pub fn insert(&mut self, index: usize, element: LanguageIdentifier) {
        self.inner.insert(index, element);
    }
}

// Due to limitation of trait, we need a nameable Iterator type.  Due to the
// lack of GATs, these have to own members instead of taking slices.
struct BundleIter {
    locales: <Vec<LanguageIdentifier> as IntoIterator>::IntoIter,
    res_ids: Vec<String>,
}

impl BundleIterator<FluentResource> for BundleIter {}

impl Iterator for BundleIter {
    type Item = FluentBundleResult<FluentResource>;

    fn next(&mut self) -> Option<Self::Item> {
        let locale = self.locales.next()?;

        let mut bundle = FluentBundle::new(vec![locale.clone()]);

        let mut errors = vec![];

        for res_id in &self.res_ids {
            let full_path = format!("./tests/resources/{}/{}", locale, res_id);
            let source = fs::read_to_string(full_path).unwrap();
            let res = match FluentResource::try_new(source) {
                Ok(res) => res,
                Err((res, err)) => {
                    errors.extend(err.into_iter().map(Into::into));
                    res
                }
            };
            bundle.add_resource(res).unwrap();
        }
        if errors.is_empty() {
            Some(Ok(bundle))
        } else {
            Some(Err((bundle, errors)))
        }
    }
}

impl BundleStream<FluentResource> for BundleIter {}

impl futures::Stream for BundleIter {
    type Item = FluentBundleResult<FluentResource>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        todo!()
    }
}

impl BundleGenerator for Locales {
    type Resource = FluentResource;
    type Iter = BundleIter;
    type Stream = BundleIter;

    fn bundles_iter(&self, res_ids: Vec<String>) -> Self::Iter {
        BundleIter {
            locales: self.inner.locales.borrow().clone().into_iter(),
            res_ids,
        }
    }

    fn bundles_stream(&self, _res_ids: Vec<String>) -> Self::Stream {
        todo!()
    }
}

#[test]
fn localization_format() {
    let resource_ids: Vec<String> = vec!["test.ftl".into(), "test2.ftl".into()];
    let locales = Locales::new(vec![langid!("pl"), langid!("en-US")]);
    let mut errors = vec![];

    let loc = Localization::with_generator(resource_ids, true, locales);

    let value = loc
        .format_value_sync("hello-world", None, &mut errors)
        .unwrap();
    assert_eq!(value, Some(Cow::Borrowed("Hello World [pl]")));

    let value = loc
        .format_value_sync("missing-message", None, &mut errors)
        .unwrap();
    assert_eq!(value, None);

    let value = loc
        .format_value_sync("hello-world-3", None, &mut errors)
        .unwrap();
    assert_eq!(value, Some(Cow::Borrowed("Hello World 3 [en]")));

    assert_eq!(errors.len(), 1);
}

#[test]
fn localization_on_change() {
    let resource_ids: Vec<String> = vec!["test.ftl".into(), "test2.ftl".into()];

    let mut locales = Locales::new(vec![langid!("en-US")]);
    let mut errors = vec![];

    let mut loc = Localization::with_generator(resource_ids, true, locales.clone());

    let value = loc
        .format_value_sync("hello-world", None, &mut errors)
        .unwrap();
    assert_eq!(value, Some(Cow::Borrowed("Hello World [en]")));

    locales.insert(0, langid!("pl"));
    loc.on_change();

    let value = loc
        .format_value_sync("hello-world", None, &mut errors)
        .unwrap();
    assert_eq!(value, Some(Cow::Borrowed("Hello World [pl]")));
}
