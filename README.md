# Fluent [![Build](https://github.com/projectfluent/fluent-rs/actions/workflows/test.yaml/badge.svg)](https://github.com/projectfluent/fluent-rs/actions/workflows/test.yaml) [![Coverage Status](https://coveralls.io/repos/github/projectfluent/fluent-rs/badge.svg?branch=main)](https://coveralls.io/github/projectfluent/fluent-rs?branch=main)

The `fluent-rs` workspace is a collection of Rust crates implementing [Project Fluent][],
a localization system designed to unleash the entire expressive power of natural language translations.

## Packages

This workspace contains the following crates:

### fluent [![crates.io](https://img.shields.io/crates/v/fluent.svg)](https://crates.io/crates/fluent)

An umbrella crate exposing the combined features of fluent-rs crates with additional convenience macros.

### fluent-bundle [![crates.io](https://img.shields.io/crates/v/fluent_bundle.svg)](https://crates.io/crates/fluent_bundle)

A low level implementation of a collection of localization messages for a single locale.

### fluent-fallback [![crates.io](https://img.shields.io/crates/v/fluent_fallback.svg)](https://crates.io/crates/fluent_fallback)

A high-level abstraction model for managing locale bundles and runtime localization lifecycle.

### fluent-resmgr [![crates.io](https://img.shields.io/crates/v/fluent_resmgr.svg)](https://crates.io/crates/fluent_resmgr)

A standalone solution for managing localization resource files and returning locale bundles.

### fluent-syntax [![crates.io](https://img.shields.io/crates/v/fluent_syntax.svg)](https://crates.io/crates/fluent_syntax)

The low level parser, AST, and serializer APIs for the Fluent Syntax.

### fluent-pseudo [![crates.io](https://img.shields.io/crates/v/fluent_pseudo.svg)](https://crates.io/crates/fluent_pseudo)

A pseudolocalization and transformation API.

### fluent-testing [![crates.io](https://img.shields.io/crates/v/fluent_testing.svg)](https://crates.io/crates/fluent_testing)

A collection of mock scenarios for testing fluent-rs components.

### intl-memoizer [![crates.io](https://img.shields.io/crates/v/fluent_testing.svg)](https://crates.io/crates/fluent_testing)

A memoizer specifically tailored for storing lazy-initialized intl formatters.

### fluent-cli

A collection of developer-oriented command line tools for Fluent.

[Project Fluent]: https://projectfluent.org
