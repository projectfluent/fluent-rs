use crate::get_test_file_path;
use std::cell::RefCell;
use std::rc::Rc;

pub enum Files {
    Mock(Vec<(String, String)>),
    Fs(Vec<(String, Option<String>)>),
}

#[derive(Clone)]
pub struct FileSource {
    pub name: String,
    pub locales: Vec<String>,
    pub path_scheme: String,
    pub files: Rc<RefCell<Files>>,
}

impl Default for FileSource {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            path_scheme: "{locale}/".to_string(),
            locales: vec!["en-US".to_string()],
            files: Rc::new(RefCell::new(Files::Mock(vec![]))),
        }
    }
}

impl FileSource {
    pub fn new<S: ToString>(
        name: S,
        path_scheme: S,
        locales: Vec<S>,
        files: Option<Vec<(S, S)>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            path_scheme: path_scheme.to_string(),
            locales: locales
                .iter()
                .map(|l| l.to_string().parse().unwrap())
                .collect(),
            files: Rc::new(RefCell::new(if let Some(files) = files {
                Files::Mock(
                    files
                        .iter()
                        .map(|(path, value)| (path.to_string(), value.to_string()))
                        .collect(),
                )
            } else {
                Files::Fs(vec![])
            })),
        }
    }

    pub fn get_file(&self, locale: &str, path: &str) -> Option<String> {
        let mut files = self.files.borrow_mut();
        let files: &mut Files = &mut files;
        match files {
            Files::Mock(files) => {
                for (p, value) in files.iter() {
                    if p == path {
                        return Some(value.to_string());
                    }
                }
                None
            }
            Files::Fs(files) => {
                for (p, value) in files.iter() {
                    if p == path {
                        return value.as_ref().map(|v| v.to_string());
                    }
                }

                let root_path =
                    get_test_file_path().join(&self.path_scheme.replace("{locale}", locale));
                let full_path = root_path.join(&path);

                if let Ok(content) = std::fs::read_to_string(full_path) {
                    files.push((path.to_string(), Some(content.clone())));
                    Some(content)
                } else {
                    files.push((path.to_string(), None));
                    None
                }
            }
        }
    }
}
