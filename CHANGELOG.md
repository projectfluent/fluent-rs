# Changelog

## Unreleased

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
