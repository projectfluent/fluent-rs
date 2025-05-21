# Changelog

## Unreleased
  - Implement NUMBER builtin
  - Adapt for new optional source positions span feature in syntax parser
  - Improve examples
  - Refactor to remove unnecessary named lifetimes
  - Cleanup docs
  - Satiate Clippy
  - Bump `smallvec` to 1.13
  - Bump `rand` to 0.9
  - Bump `self_cell` to 1.2
  - Bump `serde_yaml` to 0.9

## fluent-bundle 0.15.3 (March 16, 2024)
  - This is a 'safe harbor' release prior to bringing on non-Mozilla community maintainers
  - Implement `From<&String>` for `FluentValue`
  - Add `FluentValue.into_string` to prevent String clone
  - Fix `FluentValue::try_number` accepting numbers
  - Allow optional arguments on `FluentValue`
  - Fix behavior of `FluentArgs::set`
  - Resolve function instead in `impl ResolveValue`
  - Add type alias for concurrent `FluentBundle`
  - Fix `FluentBundle::format_pattern` lifetimes
  - Workspace: Update to Rust 2021
  - Workspace: Add various missing documentation and fix typos and links
  - Workspace: Cleanup meta-data using workspaces, use SPDX licenses, etc.
  - Workspace: Apply rustfmt and clippy lint fixes
## fluent-bundle 0.15.2 (October 25, 2021)
  - Bump `self_cell` to 0.10.

## fluent-bundle 0.15.1 (April 26, 2021)
  - Expose `resolver::errors` to allow for `ReferenceKind` matching.

## fluent-bundle 0.15.0 (February 9, 2021)
  - Document the crate.
  - Update `fluent-syntax` to 0.11.0.
  - Add `iai` benchmarks.
  - Switch `FluentArgs::add` to `FluentArgs::set`.
  - Make `FluentArgs::set` take `Into<V>`.
  - Make `FluentMessage` and `FluentAttribute` fields use getters.
  - Sort `FluentArgs` keys.
  - Turn `FluentMessage` and `FluentAttribute` to be shallow wrappers around `AST` entries.

## fluent-bundle 0.14.4 (January 31, 2021)
  - Expose `FluentResource::source()`.
  - Update `ouroboros` to 0.8.

## fluent-bundle 0.14.3 (January 24, 2021)
  - Use the `Parser::parse_runtime` in `FluentResource::try_new`.
  - Update to `fluent-syntax` 0.10.2.

## fluent-bundle 0.14.2 (January 21, 2021)
  - Switch to `FxHash` for entry hashing.

## fluent-bundle 0.14.1 (January 12, 2021)
  - Bump `ouroboros` to 0.7.

## fluent-bundle 0.14.0 (January 3, 2021)
  - Switch `FluentBundle::new` to take a `Vec<LanguageIdentifier>`.
  - Switch `rental` to `ouroboros`.
  - Add `Default` to `FluentArgs`.
  - Make `FluentError` implement `Error`.

## fluent-bundle 0.13.2 (November 11, 2020)
  - Re-add `Debug` to `FluentArgs`

## fluent-bundle 0.13.1 (September 24, 2020)
  - Replace `HashMap` based `FluentMessage` with `Vec` based one.
  - 0.13.1 brings close to 40% performance improvements over 0.12 on our benchmarks!

## fluent-bundle 0.13.0 (September 24, 2020)
  - Update to `fluent-syntax` 0.10.
  - Add `FluentBundle::write_pattern` which can write to pre-allocated buffer.
  - Get rid of `DisplayableNode` and simplify `FluentValue`.
  - Reorganize `Resolver` around `impl Write`.
  - Introduce `FluentArgs` as a struct over `Vec<FluentArg>`.
  - Introduce `FluentMessage` and `FluentAttributes`.
  - Make `FluentArgs` accept `Cow<str>` as keys.


## fluent-bundle 0.12.0 (May 6, 2020)
  - Add `Send` to `FluentType::Custom` (#173)
  - Update `intl-pluralrules` to 7.0.
  - Update `unic-langid` to 0.9.
  - Update `fluent-langneg` to 0.13.
  - Fix handling of 64bit numbers on 32bit systems.

## fluent-bundle 0.11.0 (March 10, 2020)
  - Separate out `concurrent` version of `FluentBundle`.
  - Switch FluentBundle functions to use function pointers.

## fluent-bundle 0.10.2 (February 20, 2020)
  - Update to `intl_memoizer` 0.3.0 to allow for Send+Sync on FluentBundle.

## fluent-bundle 0.10.1 (February 15, 2020)
  - Switch RefCell in FluentBundle to Mutex.

## fluent-bundle 0.10.0 (February 13, 2020)
  - Update `fluent-langneg` to 0.12.
  - Update `intl_pluralrules` to 6.0.
  - Update `unic-langid` to 0.8.
  - Introduce `intl-memoizer`.
  - Improve the ergonomics of FluentArgs.
  - Add `add_resource_overriding`.
  - Remove dependency on `failure`.
  - Switch the strategy to mitigate bomb attack to limit the number of placeables.
  - Introduce `FluentType` for custom types.
  - Improve ergonomics of `FluentNumber` and bring its features closer to ECMA402 Intl.NumberFormat.

## fluent-bundle 0.9.0 (November 26, 2019)
  - Update `unic-langid` to 0.7.
  - Update `fluent-langneg` to 0.11.
  - Update `intl_pluralrules` to 5.0.

## fluent-bundle 0.8.0 (October 3, 2019)

  - Update `unic-langid` to 0.6.
  - Update `fluent-locale` to 0.10.

## fluent-bundle 0.7.2 (October 1, 2019)

  - Update `unic-langid` to 0.5.
  - Update `fluent-locale` to 0.9.
  - Stop using macros to cut on compilation time and dependencies.

## fluent-bundle 0.7.1 (August 1, 2019)

  - Fix FluentBundle::default to use isolating by default.

## fluent-bundle 0.7.0 (August 1, 2019)

  - Turn FluentBundle to be a generic over Borrow<FluentResource> (#114)
  - Update FluentBundle to the latest API (0.14) (#120)
  - Switch to unic_langid for Language Identifier Management
  - Refactor FluentArgs (#130)
  - Add transform to FluentBundle to enable pseudolocalization (#131)
  - Refactor resolver errors to provide better fallbacking and return errors out of formatting (#93)
  - Enable FSI/PDI direction isolation (#116)
  - Add more convenience From impls for FluentValue (#108)
  - Fix `bare_trait_objects` warnings (#110)

## fluent-bundle 0.6.0 (March 26, 2019)

  - Update to fluent-syntax 0.9
  - Unify benchmark testsuite with fluent.js

## fluent-bundle 0.5.0 (January 31, 2019)

  - Update to fluent-syntax 0.8
  - Add unicode escaping
  - Align with zero-copy parser

## fluent 0.4.3 (October 13, 2018)

  - Support Sync+Send in Entry (#70)

## fluent 0.4.2 (October 1, 2018)

  - Separate lifetimes of `FluentBundle::new` and return values. (#68)

## fluent 0.4.1 (August 31, 2018)

  - Update README to make the example match  new API

## fluent 0.4.0 (August 31, 2018)

  - Rename MessageContext to FluentBundle
  - Update the FluentBundle API to match fluent.js 0.8
  - Update intl-pluralrules to 1.0
  - Add FluentBundle::format_message
  - Add FluentResource for external resource caching
  - Update fluent-syntax to 0.1.1
  - Update the signature of FluentBundle::format and FluentBundle::format_message

## fluent 0.3.1 (August 6, 2018)

  - Update `fluent-locale` to 0.4.1.
  - Switch MessageContext::locales to be an owned Vec\<String>
  - Switch FluentValue::From\<i8> to FluentValue::From\<isize>

## fluent 0.3.0 (August 3, 2018)

  - Add support for custom functions in MessageContext. (#50)
  - Switch error handling to `annotate-snippets crate`.
  - Separate `fluent` and `fluent-syntax` crates.
  - Handle cyclic references. (#55)
  - Switch parser binary to use `clap`.
  - Switch plural rules handling to `intl_pluralrules`. (#56)
  - Add `FluentValue::as_number`
  - Move `IntlPluralRules` initialization into `MessageContext::new`
  - General cleanups in line with `cargo fmt` and `cargo clippy`

## fluent 0.2.0 (February 11, 2018)

  - Support Rust 1.23 stable
  - Support Fluent 0.5 syntax
  - Dual-license Apache 2.0 and MIT

## fluent 0.1.2 (October 14, 2017)

  - Add more complex PluralRules support

## fluent 0.1.0 (October 13, 2017)

  - Support parsing Fluent Syntax 0.3.
  - Support formatting Messages and Attributes alike.
  - Support string- and Number-typed external arguments
  - Select expressions:
    - without a selector.
    - with literal strings and numbers as selector,
    - with external arguments as selector,
    - with message reference as selector (using tags).
  - Support matching numbers in select expression to plural categories.
    - Only a single mock plural rule has been implemented for now.
  - Support Attribute expressions.
  - Support Variant expressions.
  - `MessageContext::new` now takes a slice as the `locales` argument.
  - Added integration with Travis CI and Coveralls.
  - Expanded module documentation.


## fluent 0.0.1 (January 17, 2017)

  - This is the first release to be listed in the CHANGELOG.
  - Basic parser support for the FTL syntax.
  - Message references.
