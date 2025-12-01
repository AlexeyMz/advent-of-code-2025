use std::path::{Path, PathBuf};
use std::time::Instant;

pub fn get_data_path(data_path: impl AsRef<Path>) -> PathBuf {
    return Path::new("../advent-of-code-data/2025").join(data_path);
}

pub struct MeasureElapsed {
    start: Instant,
}

impl MeasureElapsed {
    pub fn start() -> MeasureElapsed {
        MeasureElapsed { start: Instant::now() }
    }

    pub fn print_measured(&mut self, name: &str) {
        println!("{name} elapsed time: {:.2?}", self.start.elapsed());
        self.start = Instant::now();
    }
}
