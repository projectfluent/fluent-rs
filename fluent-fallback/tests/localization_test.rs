use std::fs;

use fluent_bundle::{FluentBundle, FluentResource};
use fluent_fallback::{BundleGeneratorSync, SyncLocalization};
use unic_langid::{langid, LanguageIdentifier};

struct Locales {
    locales: Vec<LanguageIdentifier>,
}

impl Locales {
    pub fn new(locales: Vec<LanguageIdentifier>) -> Self {
        Self { locales }
    }

    pub fn insert(&mut self, index: usize, element: LanguageIdentifier) {
        self.locales.insert(index, element);
    }
}

// Due to limitation of trait, we need a nameable Iterator type.  Due to the
// lack of GATs, these have to own members instead of taking slices.
struct BundleIter {
    locales: <Vec<LanguageIdentifier> as IntoIterator>::IntoIter,
    resource_ids: Vec<String>,
}

impl Iterator for BundleIter {
    type Item = FluentBundle<FluentResource>;

    fn next(&mut self) -> Option<Self::Item> {
        let locale = self.locales.next()?;

        let mut bundle = FluentBundle::new(Some(&locale));

        for res_id in &self.resource_ids {
            let full_path = format!("./tests/resources/{}/{}", locale, res_id);
            let source = fs::read_to_string(full_path).unwrap();
            let res = FluentResource::try_new(source).unwrap();
            bundle.add_resource(res).unwrap();
        }
        Some(bundle)
    }
}

impl BundleGeneratorSync for Locales {
    type Resource = FluentResource;
    type Iter = BundleIter;

    fn bundles_sync(&self, resource_ids: Vec<String>) -> Self::Iter {
        BundleIter {
            locales: self.locales.clone().into_iter(),
            resource_ids,
        }
    }
}

#[test]
fn localization_format() {
    let resource_ids: Vec<String> = vec!["test.ftl".into(), "test2.ftl".into()];
    let locales = Locales::new(vec![langid!("pl"), langid!("en-US")]);

    let loc = SyncLocalization::with_generator(resource_ids, locales);

    let value = loc.format_value_sync("hello-world", None);
    assert_eq!(value, "Hello World [pl]");

    let value = loc.format_value_sync("missing-message", None);
    assert_eq!(value, "missing-message");

    let value = loc.format_value_sync("hello-world-3", None);
    assert_eq!(value, "Hello World 3 [en]");
}

#[test]
fn localization_on_change() {
    let resource_ids: Vec<String> = vec!["test.ftl".into(), "test2.ftl".into()];

    let locales = Locales::new(vec![langid!("en-US")]);

    let mut loc = SyncLocalization::with_generator(resource_ids, locales);

    let value = loc.format_value_sync("hello-world", None);
    assert_eq!(value, "Hello World [en]");

    loc.insert(0, langid!("pl"));
    loc.on_change();

    let value = loc.format_value_sync("hello-world", None);
    assert_eq!(value, "Hello World [pl]");
}
