use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use criterion::{Bencher, BenchmarkId};
use icu_calendar::DateTime;
use icu_datetime::{options::length::Time, TimeFormatter};
use icu_locid::LanguageIdentifier;
use intl_memoizer::{IntlLangMemoizer, Memoizable};

struct TF(pub TimeFormatter);

use icu_provider_blob::BlobDataProvider;
const ICU4X_DATA: &[u8] = include_bytes!(concat!(
    "/Users/zibi/projects/icu-perf/data/icu4x-1.4-datetime.postcard"
));

impl Memoizable for TF {
    type Args = (Time,);

    type Provider = icu_provider_blob::BlobDataProvider;

    /// If the construtor is fallible, than errors can be described here.
    type Error = ();

    /// This function wires together the `Args` and `Error` type to construct
    /// the intl API. In our example, there is
    fn construct(
        lang: LanguageIdentifier,
        args: Self::Args,
        provider: Option<&Self::Provider>,
    ) -> Result<Self, Self::Error> {
        Ok(Self(
            TimeFormatter::try_new_with_length_with_buffer_provider(
                provider.unwrap(),
                &lang.into(),
                args.0,
            )
            .unwrap(),
        ))
    }
}

const SETS: usize = 10;
const REPS: usize = 10;

fn construct_lang_bench(c: &mut Criterion) {
    let lang: LanguageIdentifier = "en-US".parse().unwrap();
    let provider =
        BlobDataProvider::try_new_from_static_blob(ICU4X_DATA).expect("Failed to load data");

    c.bench_with_input(
        BenchmarkId::new("construct_lang", &lang),
        &(lang, provider),
        |b, (lang, provider)| {
            b.iter(|| {
                let _ = IntlLangMemoizer::new(lang.clone(), Some(provider));
            });
        },
    );
}

fn populate_lang(c: &mut Criterion) {
    let lang: LanguageIdentifier = "en".parse().unwrap();

    let input = DateTime::try_new_gregorian_datetime(2020, 9, 1, 12, 34, 28).unwrap();
    let provider =
        BlobDataProvider::try_new_from_static_blob(ICU4X_DATA).expect("Failed to load data");
    let construct_args = (Time::Short,);

    c.bench_with_input(
        BenchmarkId::new("populate_lang", &lang),
        &(construct_args, provider),
        |b: &mut Bencher, (construct_args, provider)| {
            b.iter(|| {
                let memoizer = IntlLangMemoizer::new(lang.clone(), Some(provider));
                for _ in 0..SETS {
                    for _ in 0..REPS {
                        let _ = memoizer.with_try_get::<TF, _, _>(construct_args, |intl_example| {
                            intl_example.0.format_to_string(&input)
                        });
                    }
                }
            });
        },
    );
}

fn without_memoizer(c: &mut Criterion) {
    let lang: LanguageIdentifier = "en".parse().unwrap();
    let provider =
        BlobDataProvider::try_new_from_static_blob(ICU4X_DATA).expect("Failed to load data");
    let construct_args = (Time::Short,);

    let input = DateTime::try_new_gregorian_datetime(2020, 9, 1, 12, 34, 28).unwrap();

    c.bench_with_input(
        BenchmarkId::new("without_memoizer", &lang),
        &(construct_args, provider),
        |b: &mut Bencher, (construct_args, provider)| {
            b.iter(|| {
                for _ in 0..SETS {
                    for _ in 0..REPS {
                        let formatter = TimeFormatter::try_new_with_length_with_buffer_provider(
                            provider,
                            &lang.clone().into(),
                            construct_args.0,
                        )
                        .unwrap();
                        let _ = formatter.format(&input);
                    }
                }
            });
        },
    );
}

fn without_memoizer_hoisted(c: &mut Criterion) {
    let lang: LanguageIdentifier = "en".parse().unwrap();
    let provider =
        BlobDataProvider::try_new_from_static_blob(ICU4X_DATA).expect("Failed to load data");
    let construct_args = (Time::Short,);

    let input = DateTime::try_new_gregorian_datetime(2020, 9, 1, 12, 34, 28).unwrap();

    c.bench_with_input(
        BenchmarkId::new("without_memoizer_hoisted", &lang),
        &(construct_args, provider),
        |b: &mut Bencher, (construct_args, provider)| {
            b.iter(|| {
                for _ in 0..SETS {
                    let formatter = TimeFormatter::try_new_with_length_with_buffer_provider(
                        provider,
                        &lang.clone().into(),
                        construct_args.0,
                    )
                    .unwrap();
                    for _ in 0..REPS {
                        let _ = formatter.format(&input);
                    }
                }
            });
        },
    );
}

criterion_group!(
    benches,
    construct_lang_bench,
    populate_lang,
    without_memoizer,
    without_memoizer_hoisted
);
criterion_main!(benches);
