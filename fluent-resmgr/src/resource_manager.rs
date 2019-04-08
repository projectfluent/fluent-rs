use elsa::FrozenMap;
use fluent::{FluentBundle, FluentResource};
use std::fs;
use std::io;

pub struct BundleIterator<'l> {
    res_mgr: &'l ResourceManager,
    locales: Vec<String>,
    locales_ptr: usize,
    resource_ids: Vec<String>,
}

impl<'l> Iterator for BundleIterator<'l> {
    type Item = FluentBundle<'l>;

    fn next(&mut self) -> Option<FluentBundle<'l>> {
        if self.locales_ptr >= self.locales.len() {
            return None;
        }
        let locale = &self.locales[self.locales_ptr];

        let mut bundle = FluentBundle::new(&[locale]);
        for res_id in &self.resource_ids {
            let res = self.res_mgr.get_resource(&res_id, locale);
            bundle.add_resource(res).unwrap();
        }
        self.locales_ptr += 1;
        Some(bundle)
    }
}


fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

pub struct ResourceManager {
    resources: FrozenMap<String, Box<FluentResource>>,
    path_scheme: String,
}

impl ResourceManager {
    pub fn new(path_scheme: String) -> Self {
        ResourceManager {
            resources: FrozenMap::new(),
            path_scheme,
        }
    }

    fn get_resource(&self, res_id: &str, locale: &str) -> &FluentResource {
        let path = self.path_scheme.replace("{locale}", locale).replace("{res_id}", res_id);
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

    pub fn get_bundle(&self, locales: Vec<String>, resource_ids: Vec<String>) -> FluentBundle {
        let mut bundle = FluentBundle::new(&locales);
        for res_id in &resource_ids {
            let res = self.get_resource(res_id, &locales[0]);
            bundle.add_resource(res).unwrap();
        }
        bundle
    }

    pub fn get_bundles<'l>(&'l self, locales: Vec<String>, resource_ids: Vec<String>) -> BundleIterator<'l> {
        BundleIterator {
            res_mgr: self,
            locales,
            locales_ptr: 0,
            resource_ids,
        }
    }
}
