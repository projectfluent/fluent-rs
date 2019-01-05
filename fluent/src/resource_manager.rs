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

pub struct ResourceManager {
    resources: FrozenMap<String, Box<FluentResource>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resources: FrozenMap::new(),
        }
    }

    pub fn get_resource(&self, path: &str) -> &FluentResource {
        let resources = &self.resources;

        if resources.get(path).is_some() {
            self.resources.get(path).unwrap()
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
}
