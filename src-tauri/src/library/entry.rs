use std::path::Path;

#[derive(Clone, Debug)]
struct Entry<'a> {
    pub id: u64,
    pub path: &'a Path,
    // pub hash
    // pub thumbnail
}
