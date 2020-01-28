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
    pub fn new(lang: LanguageIdentifier, options: NumberFormatOptions) -> Result<Self, ()> {
        Ok(Self { lang, options })
    }

    pub fn format(&self, input: isize) -> String {
        format!(
            "{}: {}, MFD: {}",
            self.lang, input, self.options.minimum_fraction_digits
        )
    }
}

impl Memoizable for NumberFormat {
    type Args = (NumberFormatOptions,);
    type Error = ();
    fn construct(lang: LanguageIdentifier, args: Self::Args) -> Result<Self, Self::Error> {
        Self::new(lang, args.0)
    }
}

fn main() {
    let mut memoizer = IntlMemoizer::default();

    let lang: LanguageIdentifier = "en-US".parse().unwrap();
    {
        // Create an en-US memoizer
        let lang_memoizer = memoizer.get_for_lang(lang.clone());
        {
            let mut lang_memoizer2 = lang_memoizer.borrow_mut();

            let options = NumberFormatOptions {
                minimum_fraction_digits: 3,
                maximum_fraction_digits: 5,
            };
            let nf = lang_memoizer2.try_get::<NumberFormat>((options,)).unwrap();

            assert_eq!(&nf.format(2), "en-US: 2");
        }

        // Reuse the same en-US memoizer
        let lang_memoizer3 = memoizer.get_for_lang(lang.clone());
        {
            let mut lang_memoizer4 = lang_memoizer3.borrow_mut();

            let options2 = NumberFormatOptions {
                minimum_fraction_digits: 3,
                maximum_fraction_digits: 5,
            };
            let nf2 = lang_memoizer4.try_get::<NumberFormat>((options2,)).unwrap();

            assert_eq!(&nf2.format(2), "en-US: 2");
        }

        // Memoizer gets dropped
    }

    {
        // Here, we will construct a new lang memoizer
        let lang_memoizer = memoizer.get_for_lang(lang.clone());
        {
            let mut lang_memoizer2 = lang_memoizer.borrow_mut();

            let options = NumberFormatOptions {
                minimum_fraction_digits: 3,
                maximum_fraction_digits: 5,
            };
            let nf = lang_memoizer2.try_get::<NumberFormat>((options,)).unwrap();

            assert_eq!(&nf.format(2), "en-US: 2");
        }
    }
}
