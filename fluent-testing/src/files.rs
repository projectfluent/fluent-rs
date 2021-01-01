#[derive(Clone)]
pub struct FileSource {
    pub name: String,
    pub locales: Vec<String>,
    pub path_scheme: String,
}

impl Default for FileSource {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            path_scheme: "{locale}/".to_string(),
            locales: vec!["en-US".to_string()],
        }
    }
}

impl FileSource {
    pub fn new<S: ToString>(name: S, path_scheme: S, locales: Vec<S>) -> Self {
        Self {
            name: name.to_string(),
            path_scheme: path_scheme.to_string(),
            locales: locales
                .iter()
                .map(|l| l.to_string().parse().unwrap())
                .collect(),
        }
    }
}
