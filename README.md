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
Fluent's spec.  We're currently working towards an MVP version 0.1 with the
following scope:

  - [x] Patterns without interpolations
  - [ ] Message references
  - [ ] String-typed external arguments
  - [ ] Number-typed external arguments
  - [ ] Plural rule for English
  - [ ] Select expressions

Post-0.1 versions will provide support for:

  - [ ] Multi-line patterns
  - [ ] Quoted whitespace
  - [ ] Traits
  - [ ] Sections
  - [ ] Comments
  - [ ] Member expressions
  - [ ] Plural rules for other languages
  - [ ] Call expressions and built-in functions


Install
-------

    cargo install fluent


Develop
-------

    cargo build
    cargo test
    cargo bench
    cargo run --example simple
