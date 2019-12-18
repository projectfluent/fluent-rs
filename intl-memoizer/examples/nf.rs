use intl_memoizer::{IntlMemoizer, Memoizable};
use unic_langid::LanguageIdentifier;

struct NumberFormat {
    lang: LanguageIdentifier,
}

impl NumberFormat {
    pub fn new(lang: LanguageIdentifier) -> Self {
        println!("NEW");
        Self { lang }
    }
    pub fn format(&self) -> String {
        String::from("2.0")
    }
}

impl Memoizable for NumberFormat {
    type Args = ();
    fn construct(lang: LanguageIdentifier, args: Self::Args) -> Self {
        Self::new(lang)
    }
}

fn main() {
    let mut memoizer = IntlMemoizer::new();

    let nf = memoizer.get::<NumberFormat>("en-US".parse().unwrap(), ());
}
