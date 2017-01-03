Project Fluent
==============

This is a Rust implementation of Project Fluent, a localization framework
designed to unleash the entire expressive power of natural language
translations.

Project Fluent keeps simple things simple and makes complex things possible.
The syntax used for describing translations is easy to read and understand.  At
the same time it allows, when necessary, to represent complex concepts from
natural languages like gender, plurals, conjugations, and others.


Status
------

The implementation is in its early stages and supports only some of the Project
Fluent's spec.  Consult the [list of milestones][] for more information about
release planning and scope.

[list of milestones]: https://github.com/projectfluent/fluent-rs/milestones

Install
-------

    cargo install fluent


Develop
-------

    cargo build
    cargo test
    cargo bench
    cargo run --example simple
