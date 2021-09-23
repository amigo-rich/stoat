CREATE TABLE image (
       id INT PRIMARY KEY,
       hash TEXT UNIQUE,
       path TEXT NOT NULL
);

CREATE TABLE image_location (
       id INT PRIMARY KEY,
       name TEXT,
       latitude REAL,
       longitude REAL
);

CREATE TABLE image_rating_value (
       value INT PRIMARY KEY
);

CREATE TABLE image_ratings (
       id INT PRIMARY KEY,
       image_id INT,
       image_rating_value_value INT,
       FOREIGN KEY (image_id) REFERENCES image(id),
       FOREIGN KEY (image_rating_value_value) REFERENCES image_rating_value (value)
);

CREATE TABLE image_category_value (
       value TEXT PRIMARY KEY
);

CREATE TABLE image_categories (
       id INT PRIMARY KEY,
       image_id INT,
       image_category_value_value TEXT,
       FOREIGN KEY (image_id) REFERENCES image (id),
       FOREIGN KEY (image_category_value_value) REFERENCES image_category_value (value)
);

INSERT INTO image_rating_value (value) VALUES (1), (2), (3), (4), (5);
