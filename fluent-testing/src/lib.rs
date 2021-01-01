mod files;
mod scenarios;

pub use files::FileSource;
pub use scenarios::get_scenarios;
use std::path::PathBuf;

pub fn get_test_file_path() -> PathBuf {
    PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("resources")
}

#[cfg(feature = "sync")]
pub fn get_test_file_sync(path: &str) -> std::io::Result<String> {
    let root_path = get_test_file_path();
    let full_path = root_path.join(path);
    std::fs::read_to_string(full_path)
}

#[cfg(feature = "async")]
pub async fn get_test_file_async(path: &str) -> std::io::Result<String> {
    let root_path = get_test_file_path();
    let full_path = root_path.join(path);
    tokio::fs::read_to_string(full_path).await
}
