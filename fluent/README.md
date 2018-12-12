# Fluent

`fluent-rs` is a Rust implementation of [Project Fluent][], a localization
framework designed to unleash the entire expressive power of natural language
translations.

[![crates.io](http://meritbadge.herokuapp.com/fluent)](https://crates.io/crates/fluent)
[![Build Status](https://travis-ci.org/projectfluent/fluent-rs.svg?branch=master)](https://travis-ci.org/projectfluent/fluent-rs)
[![Coverage Status](https://coveralls.io/repos/github/projectfluent/fluent-rs/badge.svg?branch=master)](https://coveralls.io/github/projectfluent/fluent-rs?branch=master)

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
use fluent::bundle::FluentBundle;

fn main() {
    let mut bundle = FluentBundle::new(&["en-US"]);
    bundle.add_messages("hello-world = Hello, world!").unwrap();

    let (value, errors) = bundle.format("hello-world", None).unwrap();

    assert_eq!(&value, "Hello, world!");
}
```


Status
------

The implementation is in its early stages and supports only some of the Project
Fluent's spec.  Consult the [list of milestones][] for more information about
release planning and scope.

[list of milestones]: https://github.com/projectfluent/fluent-rs/milestones


Local Development
-----------------

    cargo build
    cargo test
    cargo bench
    cargo run --example hello

When submitting a PR please use  [`cargo fmt`][] (nightly).

[`cargo fmt`]: https://github.com/rust-lang-nursery/rustfmt


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

`fluent-rs` is open-source, licensed under the Apache License, Version 2.0.  We
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
 - IRC channel: [irc://irc.mozilla.org/l20n](irc://irc.mozilla.org/l20n)
