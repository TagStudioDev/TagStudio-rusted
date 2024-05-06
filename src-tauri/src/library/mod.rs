pub mod entry;
pub mod tag;

use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Location {
    path: PathBuf,
    name: Option<String>,
}

impl Location {
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[derive(Clone, Debug)]
pub struct Library {
    pub directories: Vec<Location>,
}
