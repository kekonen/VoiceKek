-- Your SQL goes here
CREATE TABLE voices (
  id SERIAL PRIMARY KEY,
  file_id VARCHAR(40) NOT NULL,
  hash_b2s VARCHAR(64),
  owner_id INTEGER NOT NULL,
  title VARCHAR(30),
  duration INTEGER,
  size INTEGER
)