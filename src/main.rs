use std::path::Path;
use stoat::{operation::Operation, run};

fn main() {
    let op = Operation::Index(Path::new("/home/rich/projects").to_path_buf());
    run(op).unwrap();
}
