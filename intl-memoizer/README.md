# IntlMemoizer

`intl-memoizer` is a crate designed to handle lazy-initialized references
to intl formatters.

The assumption is that allocating a new formatter instance is costly, and such
instance is read-only during its life time, with constructor being expensive, and
`format`/`select` calls being cheap.

In result it pays off to use a singleton to manage memoization of all instances of intl
APIs such as `PluralRules`, DateTimeFormat` etc. between all `FluentBundle` instances.

Usage
-----

The following is a high-level example of how this works, for running examples see
the [docs](https://docs.rs/intl-memoizer/)

```rust
/// Internationalization formatter should implement the Memoizable trait.
impl Memoizable for NumberFormat {
  ...
}

// The main memoizer has weak references to all of the per-language memoizers.
let mut memoizer = IntlMemoizer::default();

// The formatter memoziation happens per-locale.
let lang = "en-US".parse().expect("Failed to parse.");
let lang_memoizer: Rc<IntlLangMemoizer> = memoizer.get_for_lang(en_us);

// Run the formatter

let options: NumberFormatOptions {
    minimum_fraction_digits: 3,
    maximum_fraction_digits: 5,
};

// Format pi with the options. This will lazily construct the NumberFormat.
let pi = lang_memoizer
    .with_try_get::<NumberFormat, _, _>((options,), |nf| nf.format(3.141592653))
    .unwrap()

// The example formatter constructs a string with diagnostic information about
// the configuration.
assert_eq!(text, "3.14159");

// Running it again will use the previous formatter.
let two = lang_memoizer
    .with_try_get::<NumberFormat, _, _>((options,), |nf| nf.format(2.0))
    .unwrap()

assert_eq!(text, "2.000");
```

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
