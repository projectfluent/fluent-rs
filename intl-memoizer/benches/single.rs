use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use criterion::{Bencher, BenchmarkId};
use icu_calendar::DateTime;
use icu_datetime::{
    options::length::{Date, Time},
    // DateTimeFormatterOptions,
    DateFormatter,
    // DateTimeFormatter,
    TimeFormatter,
};
// use icu_collator::{Collator, CollatorOptions};
// use icu_decimal::{FixedDecimalFormatter, options::FixedDecimalFormatterOptions};
// use fixed_decimal::FixedDecimal;
use icu_list::{ListFormatter, ListLength};
use icu_locid::LanguageIdentifier;
use icu_plurals::{PluralRuleType, PluralRules};
use intl_memoizer::{IntlLangMemoizer, Memoizable};
use std::hint::black_box;

use icu_provider_blob::BlobDataProvider;
const ICU4X_DATA: &[u8] = include_bytes!(concat!(
    "/Users/zibi/projects/icu-perf/data/icu4x-1.4.postcard"
));

trait Testable {
    type Input;

    fn execute(&self, input: Self::Input);
}

macro_rules! define_testable_type {
    ($name:ident, $type:ident, $args:tt, $constructor:ident, $method:ident, $input:ty) => {
        define_testable_type!($name, $type, $args, $constructor);

        impl Testable for $name {
            type Input = $input;

            fn execute(&self, input: Self::Input) {
                let _ = self.0.$method(input);
            }
        }
    };

    ($name:ident, $type:ident, $args:tt, $constructor:ident, $method:ident, ref $input:ty) => {
        define_testable_type!($name, $type, $args, $constructor);

        impl Testable for $name {
            type Input = $input;

            fn execute(&self, input: Self::Input) {
                let _ = self.0.$method(&input);
            }
        }
    };

    ($name:ident, $type:ident, $args:tt, $constructor:ident) => {
        struct $name($type);

        impl Memoizable for $name {
            type Args = $args;
            type Provider = icu_provider_blob::BlobDataProvider;
            type Error = ();

            fn construct(
                lang: LanguageIdentifier,
                args: Self::Args,
                provider: Option<&Self::Provider>,
            ) -> Result<Self, Self::Error> {
                Ok(Self(
                    $type::$constructor(provider.unwrap(), &lang.into(), args.0).unwrap(),
                ))
            }
        }
    };
}

define_testable_type!(TF, TimeFormatter, (Time, ), try_new_with_length_with_buffer_provider, format_to_string, ref DateTime<icu_calendar::Gregorian>);
define_testable_type!(DF, DateFormatter, (Date, ), try_new_with_length_with_buffer_provider, format_to_string, ref DateTime<icu_calendar::AnyCalendar>);
// define_testable_type!(DTF, DateTimeFormatter, (DateTimeFormatterOptions, ), try_new_with_length_with_buffer_provider, format_to_string, ref DateTime<icu_calendar::AnyCalendar>);
define_testable_type!(
    PR,
    PluralRules,
    (PluralRuleType,),
    try_new_with_buffer_provider,
    category_for,
    usize
);
// define_testable_type!(
//     C,
//     Collator,
//     (CollatorOptions,),
//     try_new_with_buffer_provider,
//     compare,
//     &str,
//     &str,
// );
// define_testable_type!(
//     D,
//     FixedDecimalFormatter,
//     (FixedDecimalFormatterOptions,),
//     try_new_with_buffer_provider,
//     format_to_string,
//     ref FixedDecimal
// );
define_testable_type!(
    LF,
    ListFormatter,
    (ListLength,),
    try_new_and_with_length_with_buffer_provider,
    format_to_string,
    std::vec::IntoIter<String>
);

macro_rules! without_memoizer_hoisted {
    ($type:ident, $b:ident, $lang:ident, $provider:ident, $args:expr, $count:expr, $input:expr ) => {
        $b.iter(|| {
            let intl = $type::construct($lang.clone(), black_box($args), Some($provider)).unwrap();
            for _ in 0..$count {
                let _ = intl.execute($input);
            }
        })
    };
}

macro_rules! without_memoizer {
    ($type:ident, $b:ident, $lang:ident, $provider:ident, $args:expr, $count:expr, $input:expr ) => {
        $b.iter(|| {
            for _ in 0..$count {
                let intl =
                    $type::construct($lang.clone(), black_box($args), Some($provider)).unwrap();
                let _ = intl.execute($input);
            }
        })
    };
}

macro_rules! with_memoizer {
    ($type:ident, $b:ident, $lang:ident, $provider:ident, $args:expr, $count:expr, $input:expr ) => {
        $b.iter(|| {
            let memoizer =
                IntlLangMemoizer::new(black_box($lang.clone()), Some(black_box($provider)));
            for _ in 0..$count {
                let _ =
                    memoizer.with_try_get(black_box(&$args), |intl: &$type| intl.execute($input));
            }
        })
    };
}

fn bench_variants(c: &mut Criterion) {
    let lang: LanguageIdentifier = "und".parse().unwrap();

    let provider =
        BlobDataProvider::try_new_from_static_blob(ICU4X_DATA).expect("Failed to load data");

    let tf_input = DateTime::try_new_gregorian_datetime(2020, 9, 1, 12, 34, 28).unwrap();
    let tf_args = (Time::Short,);

    let pr_input = 5;
    let pr_args = (PluralRuleType::Cardinal,);

    for component in ["time", "plurals"] {
        let mut group = c.benchmark_group(component);
        let counts: &[usize] = &[0, 1, 10, 100, 1000, 10000];

        for count in counts {
            group.bench_with_input(
                BenchmarkId::new("without_memoizer_hoisted", count),
                &(count, &provider),
                |b: &mut Bencher, &(count, provider)| match component {
                    "time" => {
                        without_memoizer_hoisted!(TF, b, lang, provider, tf_args, *count, tf_input)
                    }
                    "plurals" => {
                        without_memoizer_hoisted!(PR, b, lang, provider, pr_args, *count, pr_input)
                    }
                    _ => unreachable!(),
                },
            );
            group.bench_with_input(
                BenchmarkId::new("without_memoizer", count),
                &(count, &provider),
                |b: &mut Bencher, &(count, provider)| match component {
                    "time" => without_memoizer!(TF, b, lang, provider, tf_args, *count, tf_input),
                    "plurals" => {
                        without_memoizer!(PR, b, lang, provider, pr_args, *count, pr_input)
                    }
                    _ => unreachable!(),
                },
            );
            group.bench_with_input(
                BenchmarkId::new("with_memoizer", count),
                &(count, &provider),
                |b: &mut Bencher, &(count, provider)| match component {
                    "time" => with_memoizer!(TF, b, lang, provider, tf_args, *count, tf_input),
                    "plurals" => with_memoizer!(PR, b, lang, provider, pr_args, *count, pr_input),
                    _ => unreachable!(),
                },
            );
        }
        group.finish();
    }
}

criterion_group!(benches, bench_variants,);
criterion_main!(benches);
