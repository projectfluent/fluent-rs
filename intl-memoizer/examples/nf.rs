use intl_memoizer::{IntlMemoizer, Memoizable};
use unic_langid::LanguageIdentifier;

#[derive(Clone, Hash, PartialEq, Eq)]
struct NumberFormatOptions {
    minimum_fraction_digits: usize,
    maximum_fraction_digits: usize,
}

struct NumberFormat {
    lang: LanguageIdentifier,
    options: NumberFormatOptions,
}

impl NumberFormat {
    pub fn new(lang: LanguageIdentifier, options: NumberFormatOptions) -> Self {
        Self { lang, options }
    }

    pub fn format(&self, input: isize) -> String {
        format!("{}", input)
    }
}

impl Memoizable for NumberFormat {
    type Args = (NumberFormatOptions,);
    fn construct(lang: LanguageIdentifier, args: Self::Args) -> Self {
        Self::new(lang, args.0)
    }
}

fn main() {
    let mut memoizer = IntlMemoizer::new();

    let lang: LanguageIdentifier = "en-US".parse().unwrap();

    memoizer.bump_rc(lang.clone());

    let options = NumberFormatOptions {
        minimum_fraction_digits: 3,
        maximum_fraction_digits: 5,
    };
    let nf = memoizer.get::<NumberFormat>(lang.clone(), (options,));

    assert_eq!(&nf.format(2), "2");

    memoizer.drop_rc(&lang).unwrap();
}
