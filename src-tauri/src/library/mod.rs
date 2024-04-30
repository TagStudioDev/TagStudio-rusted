use std::path::Path;

mod entry;
mod tag;

#[derive(Clone, Debug)]
pub struct Library<'a> {
    pub directories: Vec<&'a Path>,
}
