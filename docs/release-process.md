# Release Process

`fluent-rs` doesn't use strict semver yet, until the 1.0.0 release. Instead major changes are released as minor releases.

## 1. Bump version numbers

To update a package, use the `cargo-release` tool. It can be installed with `cargo install cargo-release`.

For instance to bump the `fluent-fallback` version run:

```sh
cargo release version minor -p fluent-fallback --execute
```

This will output:

```
Upgrading fluent-fallback from 0.6.0 to 0.7.0
 Updating fluent-resmgr's dependency from 0.6.0 to 0.7.0
 Updating fluent-testing's dependency from 0.6.0 to 0.7.0
```

After this the depending packages must be updated as well:

```sh
cargo release version patch -p fluent-resmgr -p fluent-testing --execute
```

## 2. Update the changelog

These are located here:

```
./fluent/CHANGELOG.md
./fluent-bundle/CHANGELOG.md
./fluent-fallback/CHANGELOG.md
./fluent-pseudo/CHANGELOG.md
./fluent-resmgr/CHANGELOG.md
./fluent-syntax/CHANGELOG.md
./fluent-testing/CHANGELOG.md
./intl-memoizer/CHANGELOG.md
```

## 3. Send in a PR for review

Get a quick sign off and merge from one other of the project maintainers and ensure the automated tests are succeeding.

## 4. Tag the release.

Go to the [create a new release](https://github.com/projectfluent/fluent-rs/releases/new) section.

The tag should be formatted like so:

`fluent-fallback@0.7.0`

Add the title, and add the changelog to the description. Before 1.0.0 release, mark as pre-release.

## 5. Publish the crates

If you do not have publish access for the crates, request access from one of the owners.

Do a dry run of the publish steps for each module.

```
cargo publish -p fluent-fallback --dry-run
cargo publish -p fluent-resmgr --dry-run
cargo publish -p fluent-testing --dry-run
```

Perform the actual publish:

```
cargo publish -p fluent-fallback
cargo publish -p fluent-resmgr
cargo publish -p fluent-testing
```

## 6. (Mozilla-only) Vendoring the crates into `mozilla-central`

a. [Search for all references](https://searchfox.org/mozilla-central/search?q=%5Efluent-%5Cw%2B+%3D+%22.*%22&path=&case=false&regexp=true) to the updated package.
a. Bump the numbers.
a. Run `mach cargo vet` to go through the vetting process and certify the results.
a. Commit `supply-chain/audits.toml`
a. Run `mach vendor rust`
a. Ensure it builds and tests pass then send it in for review.
