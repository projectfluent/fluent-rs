# Fluent Resource Manager

[![crates.io](https://img.shields.io/crates/v/fluent-resmgr.svg)](https://crates.io/crates/fluent-resmgr)
[![docs.rs](https://img.shields.io/docsrs/fluent-resmgr)](https://docs.rs/fluent-resmgr)
[![Build](https://github.com/projectfluent/fluent-rs/actions/workflows/test.yaml/badge.svg)](https://github.com/projectfluent/fluent-rs/actions/workflows/test.yaml)
[![Coverage Status](https://coveralls.io/repos/github/projectfluent/fluent-rs/badge.svg?branch=main)](https://coveralls.io/github/projectfluent/fluent-rs?branch=main)

The `fluent-rs` workspace is a collection of Rust crates implementing [Project Fluent][],
a localization system designed to unleash the entire expressive power of natural language translations.

This crate provides a standalone solution for managing localization resource files and returning locale bundles.

[Project Fluent]: https://projectfluent.org

Usage
-----

```rust
use fluent_resmgr::resource_manager::ResourceManager;

fn main() {
    let mgr = ResourceManager::new("./examples/resources/{locale}/{res_id}".into());

    let bundle = mgr.get_bundle(locales, resources);

    let value = bundle.format_value("hello-world", None);

    assert_eq!(&value, "Hello, world!");
}
```

Get Involved
------------

`fluent-rs` is open-source, licensed under both the Apache 2.0 and MIT licenses.  We
encourage everyone to take a look at our code and we'll listen to your
feedback.


Discuss
-------

We'd love to hear your thoughts on Project Fluent! Whether you're a localizer
looking for a better way to express yourself in your language, or a developer
trying to make your app localizable and multilingual, or a hacker looking for
a project to contribute to, please do get in touch on the mailing list and the
IRC channel.

 - Discourse: https://discourse.mozilla.org/c/fluent
 - Matrix channel: <a href="https://chat.mozilla.org/#/room/#fluent:mozilla.org">#fluent:mozilla.org</a>
