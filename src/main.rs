use std::path::{Path, PathBuf};
use stoat::{run, operation::Operation};

fn main() {
    let op = Operation::Build(Path::new("/home/rich/projects").to_path_buf());
    run(op).unwrap();
}
