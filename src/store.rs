use crate::category::Category;
use crate::error::Error;
use crate::image::Image;
use crate::location::Location;
use crate::rating::Rating;
use crate::schema::TABLES;
use rusqlite::{params, Connection};
use std::path::Path;

#[derive(Debug)]
pub struct Store {
    con: Connection,
}

impl Store {
    pub fn create(path: &Path) -> Result<Self, Error> {
        let con = Connection::open(path)?;
        for item in TABLES {
            let _ = con.execute(item, params![])?;
        }
        Ok(Store { con })
    }
    pub fn open(path: &Path) -> Result<Self, Error> {
        if !path.is_file() {
            return Err(Error::NotAFile(path.to_path_buf()));
        }
        let con = Connection::open(path)?;
        Ok(Store { con })
    }
    pub fn put_image(&self, hash: &str, path: &Path) -> Result<Image, Error> {
        let sql = r#"
            INSERT INTO image (hash, path)
            VALUES (?1, ?2)
        "#;
        if hash.is_empty() {
            return Err(Error::EmptyHash);
        }
        if !path.is_file() {
            return Err(Error::NotAFile(path.to_path_buf()));
        }
        let path_str_rep = path.to_str().ok_or(Error::PathConversion)?;
        let _ = self.con.execute(sql, params![hash, path_str_rep])?;
        let id = self.con.last_insert_rowid();
        let sql = r#"
            SELECT id, hash, path
            FROM image
            WHERE id = ?1
        "#;
        let result = self.con.query_row(sql, params![id], |row| {
            let path_as_string: String = row.get(2)?;
            Ok(Image {
                id: row.get(0)?,
                hash: row.get(1)?,
                path: Path::new(&path_as_string).to_path_buf(),
            })
        });
        match result {
            Ok(image) => Ok(image),
            Err(e) => Err(Error::Rusqlite(e)),
        }
    }
    pub fn put_image_location(
        &self,
        name: &str,
        latitude: f32,
        longitude: f32,
    ) -> Result<Location, Error> {
        let sql = r#"
            INSERT INTO image_location (name, latitude, longitude)
            VALUES (?1, ?2, ?3)
        "#;
        if name.is_empty() {
            return Err(Error::EmptyLocationName);
        }
        if latitude < -90.0 || latitude > 90.0 {
            return Err(Error::InvalidLatitude(latitude));
        }
        if longitude < -180.0 || longitude > 180.0 {
            return Err(Error::InvalidLongitude(longitude));
        }
        let _ = self.con.execute(sql, params![name, latitude, longitude])?;
        let id = self.con.last_insert_rowid();
        let sql = r#"
            SELECT id, name, latitude, longitude
            FROM image_location
            WHERE id = ?1
        "#;
        let result = self.con.query_row(sql, params![id], |row| {
            Ok(Location {
                id: row.get(0)?,
                name: row.get(1)?,
                latitude: row.get(2)?,
                longitude: row.get(3)?,
            })
        });
        match result {
            Ok(location) => Ok(location),
            Err(e) => Err(Error::Rusqlite(e)),
        }
    }
    pub fn put_image_rating_value(&self, rating: i64) -> Result<Rating, Error> {
        let sql = r#"
            INSERT INTO image_rating_value (value)
            VALUES (?1)
        "#;
        let _ = self.con.execute(sql, params![rating])?;
        let value = self.con.last_insert_rowid();
        Ok(Rating { value })
    }
    pub fn put_image_category_value(&self, value: &str) -> Result<Category, Error> {
        let sql = r#"
            INSERT INTO image_category_value (value)
            VALUES (?1)
        "#;
        if value.is_empty() {
            return Err(Error::EmptyCategory);
        }
        let _ = self.con.execute(sql, params![value])?;
        let sql = r#"
            SELECT value
            FROM image_category_value
            WHERE value = ?1
        "#;
        let result = self.con.query_row(sql, params![value], |row| {
            Ok(Category { value: row.get(0)? })
        });
        match result {
            Ok(category) => Ok(category),
            Err(e) => Err(Error::Rusqlite(e)),
        }
    }
    pub fn put_image_locations(&self, image: &Image, location: &Location) -> Result<i64, Error> {
        let sql = r#"
            INSERT INTO image_locations (image_id, image_location_id)
            VALUES (?1, ?2)
        "#;
        let _ = self.con.execute(sql, params![image.id, location.id])?;
        let id = self.con.last_insert_rowid();
        Ok(id)
    }
    pub fn put_image_ratings(&self, image: &Image, rating: &Rating) -> Result<i64, Error> {
        let sql = r#"
            INSERT INTO image_ratings (image_id, image_rating_value_value)
            VALUES (?1, ?2)
        "#;
        let _ = self.con.execute(sql, params![image.id, rating.value])?;
        let id = self.con.last_insert_rowid();
        Ok(id)
    }
    pub fn put_image_categories(&self, image: &Image, category: &Category) -> Result<i64, Error> {
        let sql = r#"
            INSERT INTO image_categories (image_id, image_category_value_value)
            VALUES (?1, ?2)
        "#;
        let _ = self.con.execute(sql, params![image.id, category.value])?;
        let id = self.con.last_insert_rowid();
        Ok(id)
    }
    pub fn select_image(&self) -> Result<Option<Vec<Image>>, Error> {
        let sql = r#"
            SELECT id, hash, path
            FROM image
        "#;
        let mut statement = self.con.prepare(sql)?;
        let iter = statement.query_map(params![], |row| {
            let path_rep: String = row.get(2)?;
            Ok(Image {
                id: row.get(0)?,
                hash: row.get(1)?,
                path: Path::new(&path_rep).to_path_buf(),
            })
        })?;
        let mut images: Vec<Image> = Vec::new();
        for image in iter {
            images.push(image?);
        }
        if images.is_empty() {
            return Ok(None);
        }
        Ok(Some(images))
    }
    pub fn select_image_path_like(&self, like: &str) -> Result<Option<Vec<Image>>, Error> {
        let sql = r#"
            SELECT id, hash, path
            FROM image
            WHERE LIKE(?1, path)
        "#;
        let mut statement = self.con.prepare(sql)?;
        let hack = format!("%{}%", like);
        let iter = statement.query_map(params![&hack], |row| {
            let path_rep: String = row.get(2)?;
            Ok(Image {
                id: row.get(0)?,
                hash: row.get(1)?,
                path: Path::new(&path_rep).to_path_buf(),
            })
        })?;
        let mut images: Vec<Image> = Vec::new();
        for image in iter {
            images.push(image?);
        }
        if images.is_empty() {
            return Ok(None);
        }
        Ok(Some(images))
    }
}
