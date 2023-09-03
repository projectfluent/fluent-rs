# Local testing against the Firefox codebase

Testing `fluent-rs` locally against a local copy of the Firefox codebase can be done with the following steps.

## 1. Clone Firefox locally

Bootstrap a copy of the Firefox source code without the Artifact Builds (it is needed to be able to modify the Rust dependencies).

See the [Firefox Contributors’ Quick Reference](https://firefox-source-docs.mozilla.org/contributing/contribution_quickref.html) for more information on how to achieve that on your operating system.

## 2. Delete the `fluent-rs` vendored packages

Remove the following packages from the `mozilla-unified/third_party/rust` directory:

- fluent
- fluent-bundle
- fluent-fallback
- fluent-pseudo 
- fluent-syntax 
- fluent-testing
- intl-memoizer

## 3. Bump version numbers

To avoid the `Some non-crates.io-fetched packages match published crates.io versions` error while checking the `fluent-rs` dependencies in the Firefox codebase, we must first bump the version of all the `fluent-rs` packages.

To update a package, use the `cargo-release` tool. It can be installed with `cargo install cargo-release`.

Run the following command in the `fluent-rs` directory to bump the version of all its packages:

```
cargo release version patch -p fluent -p fluent-bundle -p fluent-fallback -p fluent-pseudo -p fluent-syntax -p fluent-testing -p intl-memoizer --execute`
```

## 4. Upgrade dependencies

Now we need to upgrade all the dependencies to `fluent-rs` in the Firefox codebase to match our local copy.

- [Search for all references](https://searchfox.org/mozilla-central/search?q=%5E%28fluent%28-%5Cw%2B%29%3F%7Cintl-memoizer%29+%3D+%22.*%22&path=&case=false&regexp=true)
- Update all the references to our local packages by using `{ path = "..." }` in the `Cargo.toml` file of the impacted packages.

### Example

If both `fluent-rs` and `mozilla_unified` directories are on the same root directory, you can update the `fluent-fallback` entry from `fluent-fallback = "0.7.0"` to `fluent-fallback = { path = "../../../../../fluent-rs/fluent-fallback" }` in the `mozilla-unified/intl/l10n/rust/l10nregistry-ffi/Cargo.toml` file.

## 5. Check the local `fluent-rs` dependencies

It is done by running `./mach vendor rust` at the root of the `mozilla-unified` directory. If the `./mach vendor rust` command runs without any problems, you're good to go!

### Vetting a dependency

If you see an error similar to `intel-memoizer type-map = "0.5" must be vetted (current vetted version at 0.4) =`, it means that the current vetted version of the `type-map` package is `0.4` but we're using the `0.5` version in `fluent-rs`.

It can be fixed in the `supply-chain/config.toml` file by bumping the version in the `[[exemptions.type-map]]` to `0.5`.
