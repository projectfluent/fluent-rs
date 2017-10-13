# Fluent

**Fluent is a localization library designed to improve how software is translated.**

[![crates.io](http://meritbadge.herokuapp.com/fluent)](https://crates.io/crates/fluent)
[![Build Status](https://travis-ci.org/projectfluent/fluent-rs.svg?branch=master)](https://travis-ci.org/projectfluent/fluent-rs)
[![Coverage Status](https://coveralls.io/repos/github/projectfluent/fluent-rs/badge.svg?branch=master)](https://coveralls.io/github/projectfluent/fluent-rs?branch=master)

Introduction
------------

This is a Rust implementation of Project Fluent, a localization framework
designed to unleash the entire expressive power of natural language
translations.

Project Fluent keeps simple things simple and makes complex things possible.
The syntax used for describing translations is easy to read and understand.  At
the same time it allows, when necessary, to represent complex concepts from
natural languages like gender, plurals, conjugations, and others.

Installation
------------

```toml
[dependencies]
fluent = "0.1.0"
```

Usage
-----

```rust
extern crate fluent;

use fluent::MessageContext;

let ctx = MessageContext::new(&["en-US"]);
ctx.add_messages("hello-world = Hello WOrld in Fluent!");

let value = ctx.format("hello-world").unwrap();
println!(value);
```

See [docs.rs][] for more examples.

[docs.rs]: https://docs.rs/fluent/

Status
------

The implementation is in its early stages and supports only some of the Project
Fluent's spec.  Consult the [list of milestones][] for more information about
release planning and scope.

[list of milestones]: https://github.com/projectfluent/fluent-rs/milestones

Develop
-------

    cargo build
    cargo test
    cargo bench
    cargo run --example simple
