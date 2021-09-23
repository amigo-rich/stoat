use std::path::PathBuf;

#[derive(Debug)]
pub struct Image {
    pub hash: String,
    pub id: i64,
    pub path: PathBuf,
}
