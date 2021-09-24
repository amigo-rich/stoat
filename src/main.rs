use std::path::Path;
use stoat::{run, operation::Operation};

fn main() {
    let op = Operation::Index(Path::new("/home/rich/projects").to_path_buf());
    run(op).unwrap();
}
