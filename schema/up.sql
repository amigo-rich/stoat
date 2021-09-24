CREATE TABLE image (
       id INTEGER PRIMARY KEY,
       hash TEXT NOT NULL,
       path TEXT NOT NULL
);

CREATE TABLE image_location (
       id INTEGER PRIMARY KEY,
       name TEXT,
       latitude REAL,
       longitude REAL
);

CREATE TABLE image_locations (
       id INTEGER PRIMARY KEY,
       image_id INTEGER,
       image_location_id INTEGER,
       FOREIGN KEY (image_id) REFERENCES image (id),
       FOREIGN KEY (image_location_id) REFERENCES image_location (id)
);

CREATE TABLE image_rating_value (
       value INTEGER PRIMARY KEY
);

CREATE TABLE image_ratings (
       id INTEGER PRIMARY KEY,
       image_id INTEGER,
       image_rating_value_value INTEGER,
       FOREIGN KEY (image_id) REFERENCES image(id),
       FOREIGN KEY (image_rating_value_value) REFERENCES image_rating_value (value)
);

CREATE TABLE image_category_value (
       value TEXT PRIMARY KEY
);

CREATE TABLE image_categories (
       id INTEGER PRIMARY KEY,
       image_id INTEGER,
       image_category_value_value TEXT,
       FOREIGN KEY (image_id) REFERENCES image (id),
       FOREIGN KEY (image_category_value_value) REFERENCES image_category_value (value)
);

INSERT INTO image_rating_value (value) VALUES (1), (2), (3), (4), (5);
