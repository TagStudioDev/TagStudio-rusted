use std::path::Path;

pub mod entry;
pub mod tag;

#[derive(Clone, Debug)]
pub struct Library<'a> {
    pub directories: Vec<&'a Path>,
}
