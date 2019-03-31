use elsa::FrozenMap;
use fluent::{Fbi, FluentBundleIterator};
use fluent::{FluentBundle, FluentResource};
use std::fs;
use std::io;

pub struct BundleIterator<'l> {
    res_mgr: &'l ResourceManager,
    locales: Vec<String>,
    paths: Vec<String>,
    used: bool,
}

impl<'l> Iterator for BundleIterator<'l> {
    type Item = FluentBundle<'l>;

    fn next(&mut self) -> Option<FluentBundle<'l>> {
        if self.used {
            return None;
        }

        let mut bundle = FluentBundle::new(&self.locales);
        for path in &self.paths {
            let res = self.res_mgr.get_resource(&path);
            bundle.add_resource(res).unwrap();
        }
        self.used = true;
        Some(bundle)
    }
}

impl<'l> FluentBundleIterator<'l> for BundleIterator<'l> {}

fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

pub struct ResourceManager {
    resources: FrozenMap<String, Box<FluentResource>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resources: FrozenMap::new(),
        }
    }

    fn get_resource(&self, path: &str) -> &FluentResource {
        if let Some(res) = self.resources.get(path) {
            res
        } else {
            let string = read_file(path).unwrap();
            let res = match FluentResource::try_new(string) {
                Ok(res) => res,
                Err((res, _err)) => res,
            };
            self.resources.insert(path.to_string(), Box::new(res))
        }
    }

    pub fn get_bundle(&self, locales: &[String], paths: &Vec<String>) -> FluentBundle {
        let mut bundle = FluentBundle::new(locales);
        for path in paths {
            let res = self.get_resource(path);
            bundle.add_resource(res).unwrap();
        }
        bundle
    }

    pub fn get_bundles<'l>(&'l self, locales: Vec<String>, paths: Vec<String>) -> Fbi<'l> {
        Box::new(BundleIterator {
            res_mgr: self,
            locales,
            paths,
            used: false,
        })
    }
}
