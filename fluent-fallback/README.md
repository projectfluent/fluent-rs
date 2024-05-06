# Fluent

`fluent-fallback` is a Rust implementation of the [Project Fluent][] higher level API.

The `Localization` struct encapsulates a persistent localization context providing
language fallbacking. The instance remains available throughout the whole life cycle of
the corresponding UI, reacting to events such as locale changes, resource updates etc.

The API can be used directly, or can serve as an example of state manager for `fluent-bundle` and `fluent-resmgr`.

[![crates.io](https://img.shields.io/crates/v/fluent-fallback.svg)](https://crates.io/crates/fluent-fallback)
[![Build](https://github.com/projectfluent/fluent-rs/actions/workflows/test.yaml/badge.svg)](https://github.com/projectfluent/fluent-rs/actions/workflows/test.yaml)
[![Coverage Status](https://coveralls.io/repos/github/projectfluent/fluent-rs/badge.svg?branch=main)](https://coveralls.io/github/projectfluent/fluent-rs?branch=main)

Project Fluent keeps simple things simple and makes complex things possible.
The syntax used for describing translations is easy to read and understand.  At
the same time it allows, when necessary, to represent complex concepts from
natural languages like gender, plurals, conjugations, and others.

[Documentation][]

[Project Fluent]: http://projectfluent.org
[Documentation]: https://docs.rs/fluent/

Usage
-----

```rust
use fluent_fallback::Localization;

fn main() {
    // generate_messages is a closure that returns an iterator over FluentBundle
    // instances.
    let loc = Localization::new(vec!["simple.ftl".into()], generate_messages);

    let value = bundle.format_value("hello-world", None);

    assert_eq!(&value, "Hello, world!");
}
```


Status
------

The implementation is in its early stages and supports only some of the Project
Fluent's spec.  Consult the [list of milestones][] for more information about
release planning and scope.

[list of milestones]: https://github.com/projectfluent/fluent-rs/milestones


Learn the FTL syntax
--------------------

FTL is a localization file format used for describing translation resources.
FTL stands for _Fluent Translation List_.

FTL is designed to be simple to read, but at the same time allows to represent
complex concepts from natural languages like gender, plurals, conjugations, and
others.

    hello-user = Hello, { $username }!

[Read the Fluent Syntax Guide][] in order to learn more about the syntax.  If
you're a tool author you may be interested in the formal [EBNF grammar][].

[Read the Fluent Syntax Guide]: http://projectfluent.org/fluent/guide/
[EBNF grammar]: https://github.com/projectfluent/fluent/tree/master/spec


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
