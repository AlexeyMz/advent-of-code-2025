use std::path::{Path, PathBuf};

pub fn get_data_path(data_path: impl AsRef<Path>) -> PathBuf {
    return Path::new("../advent-of-code-data/2025").join(data_path);
}
