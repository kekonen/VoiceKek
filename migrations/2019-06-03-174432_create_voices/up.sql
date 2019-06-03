-- Your SQL goes here
CREATE TABLE voices (
  id SERIAL PRIMARY KEY,
  file_id VARCHAR(31) NOT NULL,
  hash_sha1 VARCHAR(40) NOT NULL,
  owner_id VARCHAR(9) NOT NULL,
  title VARCHAR(20) NOT NULL,
  duration INTEGER,
  size INTEGER
)