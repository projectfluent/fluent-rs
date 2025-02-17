extern crate alloc;

// to regerate the icu4x_data module, run from the root of the project:
// cargo build --examples
// rm -r fluent-bundle/src/icu4x_data
// icu4x-datagen --keys-for-bin target/debug/examples/functions --format mod --locales full -o fluent-bundle/src/icu4x_data
// for more information: https://github.com/unicode-org/icu4x/blob/79480dfbafdedf6cc810e4bfb0770f3268ff86ab/tutorials/data_management.md
include!("./icu4x_data/mod.rs");
impl_any_provider!(BakedDataProvider);

/// ICU data provider
///
/// Fluent uses ICU4X data for formatting purposes, like number formatting.
/// Use FluentIcuDataProvider to add all ICU data needed by fluent-bundle.
/// You can bring your own provider if you don't need all locales, or have one in your project already.
///
/// Example:
/// ```
/// use fluent_bundle::{FluentBundle, FluentIcuDataProvider, FluentResource};
/// use unic_langid::langid;
///
/// let langid = langid!("en-US");
/// let mut bundle: FluentBundle<FluentResource> = FluentBundle::new(vec![langid]);
/// bundle.set_icu_data_provider(Box::new(FluentIcuDataProvider));
/// ```
pub use BakedDataProvider as FluentIcuDataProvider;
