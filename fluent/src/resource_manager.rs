use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::resource::FluentResource;
use std::cell::UnsafeCell;
use std::collections::hash_map::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::ops::Deref;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

unsafe trait Allocated: Deref {}

unsafe impl Allocated for String {}
unsafe impl<T> Allocated for Box<T> {}

struct FrozenMap<T> {
    map: UnsafeCell<HashMap<String, T>>,
}

impl<T: Allocated> FrozenMap<T> {
    // under no circumstances implement delete() on this
    // under no circumstances return an &T
    pub fn new() -> Self {
        Self {
            map: UnsafeCell::new(Default::default()),
        }
    }

    pub fn insert(&self, k: String, v: T) -> &T::Target {
        unsafe {
            let map = self.map.get();
            &*(*map).entry(k).or_insert(v)
        }
    }

    pub fn get(&self, k: &str) -> Option<&T::Target> {
        unsafe {
            let map = self.map.get();
            (*map).get(k).map(|x| &**x)
        }
    }
}

pub struct ResourceManager<'mgr> {
    strings: FrozenMap<String>,
    resources: FrozenMap<Box<FluentResource<'mgr>>>,
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

    pub fn get_bundle(
        &'mgr self,
        locales: &Vec<String>,
        paths: &Vec<String>,
    ) -> FluentBundle<'mgr> {
        let mut bundle = FluentBundle::new(locales);
        for path in paths {
            let res = self.get_resource(path);
            bundle.add_resource(res).unwrap();
        }
        return bundle;
    }
}
