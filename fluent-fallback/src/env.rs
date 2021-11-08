use unic_langid::LanguageIdentifier;

pub trait LocalesProvider {
    fn locales(&self) -> <Vec<LanguageIdentifier> as IntoIterator>::IntoIter;
}

impl LocalesProvider for Vec<LanguageIdentifier> {
    fn locales(&self) -> <Vec<LanguageIdentifier> as IntoIterator>::IntoIter {
        self.clone().into_iter()
    }
}
