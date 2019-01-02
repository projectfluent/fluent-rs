use elsa::FrozenMap;
use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::resource::FluentResource;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub struct ResourceManager<'mgr> {
    strings: FrozenMap<String, String>,
    resources: FrozenMap<String, Box<FluentResource<'mgr>>>,
}

impl<'mgr> ResourceManager<'mgr> {
    pub fn new() -> Self {
        ResourceManager {
            strings: FrozenMap::new(),
            resources: FrozenMap::new(),
        }
    }

    pub fn get_resource(&'mgr self, path: &str) -> &'mgr FluentResource<'mgr> {
        let strings = &self.strings;

        if strings.get(path).is_some() {
            self.resources.get(path).unwrap()
        } else {
            let string = read_file(path).unwrap();
            let val = self.strings.insert(path.to_string(), string);
            let res = match FluentResource::from_str(val) {
                Ok(res) => res,
                Err((res, _err)) => res,
            };
            self.resources.insert(path.to_string(), Box::new(res))
        }
    }

    pub fn get_bundle(&'mgr self, locales: &[String], paths: &[String]) -> FluentBundle<'mgr> {
        let mut bundle = FluentBundle::new(locales);
        for path in paths {
            let res = self.get_resource(path);
            bundle.add_resource(res).unwrap();
        }
        bundle
    }
}
