-- Your SQL goes here
CREATE TABLE voices (
  id SERIAL PRIMARY KEY,
  file_id VARCHAR(31) NOT NULL,
  hash_sha1 VARCHAR(40),
  owner_id INTEGER NOT NULL,
  title VARCHAR(30),
  duration INTEGER,
  size INTEGER
)