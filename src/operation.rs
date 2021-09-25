use std::path::PathBuf;

#[derive(Debug)]
pub enum Operation<'a> {
    AddCategory(&'a str),
    AddLocation(&'a str, f32, f32),
    Index(PathBuf),
    List,
    ListLike(&'a str),
}
