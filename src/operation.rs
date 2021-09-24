use std::path::PathBuf;

#[derive(Debug)]
pub enum Operation<'a> {
    AddCategory(&'a str),
    Index(PathBuf),
    List,
    ListLike(&'a str),
}
