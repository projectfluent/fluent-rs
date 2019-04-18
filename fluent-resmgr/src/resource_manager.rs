use elsa::FrozenMap;
use fluent::{FluentBundle, FluentResource};
use std::fs;
use std::io;
use std::iter::from_fn;

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
        let path = self
            .path_scheme
            .replace("{locale}", locale)
            .replace("{res_id}", res_id);
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

    pub fn get_bundles<'l>(
        &'l self,
        locales: Vec<String>,
        resource_ids: Vec<String>,
    ) -> impl Iterator<Item = FluentBundle<'l>> {
        let res_mgr = self;
        let mut ptr = 0;

        from_fn(move || {
            locales.get(ptr).map(|locale| {
                ptr += 1;
                let mut bundle = FluentBundle::new(&[locale]);
                for res_id in &resource_ids {
                    let res = res_mgr.get_resource(&res_id, locale);
                    bundle.add_resource(res).unwrap();
                }
                bundle
            })
        })
    }
}
