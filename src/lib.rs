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
        Operation::AddCategory(category) => {
            let _ = store.put_image_category_value(category)?;
        }
        Operation::AddLocation(name, latitude, longitude) => {
            let _ = store.put_image_location(name, latitude, longitude)?;
        }
        Operation::Index(pb) => {
            let results = build_path_hash_vec_from_path(pb)?;
            for result in results {
                store.put_image(&result.0, &result.1)?;
            }
        }
        Operation::List => {
            let maybe_images = store.select_image()?;
            if let Some(images) = maybe_images {
                for image in images {
                    println!("{:?}", image);
                }
            }
        }
        Operation::ListLike(value) => {
            let maybe_images = store.select_image_path_like(value)?;
            if let Some(images) = maybe_images {
                for image in images {
                    println!("{:?}", image);
                }
            }
        }
    }
    Ok(())
}
