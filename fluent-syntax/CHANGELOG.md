# Changelog

## Unreleased

  - …

## fluent-syntax 0.10.3 (January 31, 2021)
  - A bunch of microoptimizations driven by the ioi benchmark.

## fluent-syntax 0.10.2 (January 24, 2021)
  - Add `parse_runtime` method on `Parser` which skips comments.
  - Fix handling of empty CRLF lines in mutliline patterns.

## fluent-syntax 0.10.1 (January 3, 2021)
  - Make `ParserError` `Clone`.
  - Apply `clippy` lints.

## fluent-syntax 0.10.0 (September 24, 2020)
  - Refactored AST to be generic over S which enables sliced, or owned ASTs.
  - Simplified the AST to get it closer to reference AST.
  - Refactored the parser to be composable.
  - Moved serde derives onto AST nodes behind `serde` optional feature.
  - Around 8-10% performance improvement on parsing "browser" and "preferences" contexts.

## fluent-syntax 0.9.3 (March 4, 2020)
  - Move JSON serialization from tests to source code behind the feature flag.
  - Fix a minor syntax issue that caused the parser to recognize a comment ending on EOF as Junk.
  - Add context benchmarks for Firefox contexts - browser and preferences.

## fluent-syntax 0.9.2 (February 13, 2020)
  - Import updated tests from the reference parser.
  - Minor parser improvements to align with new tests.

## fluent-syntax 0.9.1 (November 26, 2019)
  - Dependency updates.
  - Better test coverage.

## fluent-syntax 0.9.0 (March 26, 2019)
  - Update to Fluent Syntax 0.9
  - Unify benchmark testsuite with fluent.js

## fluent-syntax 0.8.0 (January 31, 2019)
  - Update to Fluent Syntax 0.8
  - Switch to zero-copy parser
  - Start using reference FTL fixtures in tests
  - Switch to criterion for benchmarks
  - Rust 2018 edition

## fluent-syntax 0.1.1 (August 29, 2018)

  - enable ParserError to be compared.

## fluent-syntax 0.1.0 (July 29, 2018)

  - Initial release of the standalone fluent-syntax.
    Based on fluent 0.2.0, and syntax 0.5
