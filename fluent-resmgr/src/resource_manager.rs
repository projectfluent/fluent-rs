use elsa::FrozenMap;
use fluent_bundle::{FluentBundle, FluentResource};
use fluent_fallback::{
    generator::{BundleGenerator, FluentBundleResult},
    types::ResourceId,
};
use futures::stream::Stream;
use rustc_hash::FxHashSet;
use std::fs;
use std::io;
use std::iter;
use unic_langid::LanguageIdentifier;

fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

/// [ResourceManager] provides a standalone solution for managing localization resources which
/// can be used by `fluent-fallback` or other higher level bindings.
pub struct ResourceManager {
    resources: FrozenMap<String, Box<FluentResource>>,
    path_scheme: String,
}

impl ResourceManager {
    /// Create a new and empty [`ResourceManager`]. As resources are added they will be
    /// retained in the `resources` [`FrozenMap`]. The `path_scheme` argument defines
    /// how the files are organized.
    ///
    /// For instance `"./translations/{locale}/{res_id}"` will load files with the
    /// following structure:
    ///
    /// .
    /// └── translations
    ///     ├── en-US
    ///     │   ├── app.ftl
    ///     │   └── errors.ftl
    ///     └── pl
    ///         ├── app.ftl
    ///         └── errors.ftl
    ///
    pub fn new(path_scheme: String) -> Self {
        ResourceManager {
            resources: FrozenMap::new(),
            path_scheme,
        }
    }

    /// Returns a [`FluentResource`], by either reading the file and loading it into
    /// memory, or retrieving it from an in-memory cache.
    fn get_resource(&self, resource_id: &str, locale: &str) -> &FluentResource {
        let path = self
            .path_scheme
            .replace("{locale}", locale)
            .replace("{res_id}", resource_id);
        if let Some(res) = self.resources.get(&path) {
            res
        } else {
            let string = read_file(&path).unwrap();
            let res = match FluentResource::try_new(string) {
                Ok(res) => res,
                Err((res, _err)) => res,
            };
            self.resources.insert(path.to_string(), Box::new(res))
        }
    }

    /// Gets a [`FluentBundle`] from a list of resources. The bundle will only contain the
    /// resources from the first locale in the locales list. The other locales will be
    /// stored in the [`FluentBundle`] and will only be used for custom formatters such
    /// date time format, or plural rules. The message formatting will not fall back
    /// to other locales.
    pub fn get_bundle(
        &self,
        locales: Vec<LanguageIdentifier>,
        resource_ids: Vec<String>,
    ) -> FluentBundle<&FluentResource> {
        let mut bundle = FluentBundle::new(locales.clone());
        for res_id in &resource_ids {
            println!("res_id {:?}", res_id);
            let res = self.get_resource(res_id, &locales[0].to_string());
            bundle.add_resource(res).unwrap();
        }
        bundle
    }

    /// Returns an iterator for a [`FluentBundle`] for each locale provided. Each
    /// iteration will load all of the resources for that single locale. i18n formatters
    /// such as date time format and plural rules will ignore the list of locales,
    /// unlike `get_bundle` and only use the single locale of the bundle.
    pub fn get_bundles(
        &self,
        locales: Vec<LanguageIdentifier>,
        resource_ids: Vec<String>,
    ) -> impl Iterator<Item = FluentBundle<&FluentResource>> {
        let res_mgr = self;
        let mut idx = 0;

        iter::from_fn(move || {
            locales.get(idx).map(|locale| {
                idx += 1;
                let mut bundle = FluentBundle::new(vec![locale.clone()]);
                for resource_id in &resource_ids {
                    let resource = res_mgr.get_resource(resource_id, &locale.to_string());
                    bundle.add_resource(resource).unwrap();
                }
                bundle
            })
        })
    }
}

// Due to limitation of trait, we need a nameable Iterator type.  Due to the
// lack of GATs, these have to own members instead of taking slices.
pub struct BundleIter {
    locales: <Vec<LanguageIdentifier> as IntoIterator>::IntoIter,
    res_ids: FxHashSet<ResourceId>,
}

impl Iterator for BundleIter {
    type Item = FluentBundleResult<FluentResource>;

    fn next(&mut self) -> Option<Self::Item> {
        let locale = self.locales.next()?;

        let mut bundle = FluentBundle::new(vec![locale.clone()]);

        for res_id in self.res_ids.iter() {
            let full_path = format!("./tests/resources/{}/{}", locale, res_id);
            let source = fs::read_to_string(full_path).unwrap();
            let res = FluentResource::try_new(source).unwrap();
            bundle.add_resource(res).unwrap();
        }
        Some(Ok(bundle))
    }
}

// TODO - These need to be implemented.
// https://github.com/projectfluent/fluent-rs/issues/281

// coverage(off)
impl Stream for BundleIter {
    type Item = FluentBundleResult<FluentResource>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        todo!()
    }
}

impl BundleGenerator for ResourceManager {
    type Resource = FluentResource;
    type LocalesIter = std::vec::IntoIter<LanguageIdentifier>;
    type Iter = BundleIter;
    type Stream = BundleIter;

    fn bundles_iter(
        &self,
        locales: Self::LocalesIter,
        res_ids: FxHashSet<ResourceId>,
    ) -> Self::Iter {
        BundleIter { locales, res_ids }
    }

    fn bundles_stream(
        &self,
        _locales: Self::LocalesIter,
        _res_ids: FxHashSet<ResourceId>,
    ) -> Self::Stream {
        todo!()
    }
}
// coverage(on)

#[cfg(test)]
mod test {
    use super::*;
    use unic_langid::langid;

    #[test]
    fn caching() {
        let res_mgr = ResourceManager::new("./tests/resources/{locale}/{res_id}".into());

        let _bundle = res_mgr.get_bundle(vec![langid!("en-US")], vec!["test.ftl".into()]);
        let res_1 = res_mgr.get_resource("test.ftl", "en-US");

        let _bundle2 = res_mgr.get_bundle(vec![langid!("en-US")], vec!["test.ftl".into()]);
        let res_2 = res_mgr.get_resource("test.ftl", "en-US");

        assert!(
            std::ptr::eq(res_1, res_2),
            "The resources are cached in memory and reference the same thing."
        );
    }

    // TODO - This API should return a Result instead.
    // https://github.com/projectfluent/fluent-rs/issues/278
    #[test]
    #[should_panic]
    fn get_resource_error() {
        let res_mgr = ResourceManager::new("./tests/resources/{locale}/{res_id}".into());

        let _bundle = res_mgr.get_bundle(vec![langid!("en-US")], vec!["test.ftl".into()]);
        res_mgr.get_resource("nonexistent.ftl", "en-US");
    }

    // TODO - This API should return a Result instead.
    // https://github.com/projectfluent/fluent-rs/issues/278
    #[test]
    #[should_panic]
    fn get_bundle_error() {
        let res_mgr = ResourceManager::new("./tests/resources/{locale}/{res_id}".into());
        let _bundle = res_mgr.get_bundle(vec![langid!("en-US")], vec!["nonexistent.ftl".into()]);
    }

    // TODO - Syntax errors should be surfaced.
    // https://github.com/projectfluent/fluent-rs/issues/280
    #[test]
    fn get_bundle_ignores_errors() {
        // Note that this is the behavior at the time of the writing of this test.
        let res_mgr = ResourceManager::new("./tests/resources/{locale}/{res_id}".into());
        let bundle = res_mgr.get_bundle(
            vec![langid!("en-US")],
            vec!["test.ftl".into(), "invalid.ftl".into()],
        );

        let mut errors = vec![];
        let msg = bundle.get_message("hello-world").expect("Message exists");
        let pattern = msg.value().expect("Message has a value");
        let value = bundle.format_pattern(&pattern, None, &mut errors);
        assert_eq!(value, "Hello World");

        let mut errors = vec![];
        let msg = bundle.get_message("valid-message").expect("Message exists");
        let pattern = msg.value().expect("Message has a value");
        let value = bundle.format_pattern(&pattern, None, &mut errors);
        assert_eq!(value, "This is a valid message");

        // Syntax errors are not surfaced.
    }
}
