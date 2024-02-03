# Fluent [![Build](https://github.com/projectfluent/fluent-rs/actions/workflows/test.yaml/badge.svg)](https://github.com/projectfluent/fluent-rs/actions/workflows/test.yaml) [![Coverage Status](https://coveralls.io/repos/github/projectfluent/fluent-rs/badge.svg?branch=main)](https://coveralls.io/github/projectfluent/fluent-rs?branch=main)

`fluent-rs` is a collection of Rust crates implementing [Project Fluent](https://projectfluent.org).

## Packages

The crates perform the following functions:

### fluent [![crates.io](https://img.shields.io/crates/v/fluent.svg)](https://crates.io/crates/fluent)

Umbrella crate combining crates that are ready to be used in production.

### fluent-syntax [![crates.io](https://img.shields.io/crates/v/fluent_syntax.svg)](https://crates.io/crates/fluent_syntax)

Low level Fluent Syntax AST and parser API.

### fluent-bundle [![crates.io](https://img.shields.io/crates/v/fluent_bundle.svg)](https://crates.io/crates/fluent_bundle)

Implementation of the low-level Fluent Localization System providing localization capabilities for any Rust project.

### fluent-fallback [![crates.io](https://img.shields.io/crates/v/fluent_fallback.svg)](https://crates.io/crates/fluent_fallback)

Implementation of the high-level Fluent Localization System providing localization capabilities for any Rust project.

### fluent-resmgr [![crates.io](https://img.shields.io/crates/v/fluent_resmgr.svg)](https://crates.io/crates/fluent_resmgr)

Resource Manager for localization resources.

### fluent-cli

Collection of command line tools for Fluent.

## Running the project

Each `fluent-*` directory works with the typical `cargo` commands. In addition there are some general `cargo-make` commands that can be run. First install `cargo-make` via `cargo install --force cargo-make`. The commands are documented in [Makefile.toml](Makefile.toml).

### Tests

To run all of the tests for the repo run:

```sh
cargo make test
```

For local code coverage reports run:

```sh
# Install the tools first if you haven't done so. The llvm tools must be available
# on the path for this to work correctly.
cargo make install-tools

# Then coverage can be run like so:
cargo make coverage
```
