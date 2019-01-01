use elsa::FrozenMap;
use crate::bundle::FluentBundle;
use crate::resource::FluentResource;
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

impl<'mgr> Clone for ResourceManager<'mgr> {
    fn clone(&self) -> ResourceManager<'mgr> {
        unimplemented!()
    }
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
            return self.resources.get(path).unwrap();
        } else {
            let string = read_file(path).unwrap();
            let val = self.strings.insert(path.to_string(), string);
            let res = match FluentResource::from_string(val) {
                Ok(res) => res,
                Err((res, _err)) => res,
            };
            self.resources.insert(path.to_string(), Box::new(res))
        }
    }

    pub fn get_resource_for_string(&'mgr self, string: &str) -> &'mgr FluentResource<'mgr> {
        let strings = &self.strings;

        if strings.get(string).is_some() {
            return self.resources.get(string).unwrap();
        } else {
            let val = self.strings.insert(string.to_string(), string.to_owned());
            let res = match FluentResource::from_string(val) {
                Ok(res) => res,
                Err((res, _err)) => res,
            };
            self.resources.insert(string.to_string(), Box::new(res))
        }
    }

    pub fn get_bundle(
        &'mgr self,
        locales: &Vec<String>,
        paths: &Vec<String>,
    ) -> FluentBundle<'mgr> {
        let mut bundle = FluentBundle::new(locales, Some(&self));
        for path in paths {
            let res = self.get_resource(path);
            bundle.add_resource(res).unwrap();
        }
        return bundle;
    }
}
