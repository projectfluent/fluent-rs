use elsa::FrozenMap;
use fluent::{FluentBundle, FluentResource};
use fluent::{FluentBundleIterator, Fbi};
use std::fs;
use std::io;

pub struct BundleIterator<'l> {
    test: &'l str
}

impl<'l> Iterator for BundleIterator<'l> {
    type Item = FluentBundle<'l>;

    fn next(&mut self) -> Option<FluentBundle<'l>> {
        None
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

    pub fn get_bundles<'l>(&mut self, locales: Vec<&str>, paths: Vec<&str>) -> Fbi {
        Box::new(BundleIterator {
            test: "foo"
        })
    }
}
