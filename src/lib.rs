mod build;
use build::build_path_hash_vec_from_path;
mod category;
mod error;
use error::Error;
mod image;
mod location;
pub mod operation;
use operation::Operation;
mod rating;
use std::path::Path;
mod schema;
mod store;
use store::Store;

pub fn run(operation: Operation) -> Result<(), Error> {
    let path = Path::new("test.sqlite");
    let store = match path.is_file() {
        true => Store::open(path)?,
        false => Store::create(path)?,
    };
    match operation {
        Operation::Index(pb) => {
            let results = build_path_hash_vec_from_path(pb)?;
            for result in results {
                store.put_image(&result.0, &result.1)?;
            }
        }
    }
    Ok(())
}
