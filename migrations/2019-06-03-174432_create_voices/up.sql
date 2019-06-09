-- Your SQL goes here
CREATE TABLE voices (
  id SERIAL PRIMARY KEY,
  file_id VARCHAR(40) UNIQUE NOT NULL,
  hash_b2s VARCHAR(64),
  owner_id INTEGER NOT NULL,
  title VARCHAR(30),
  duration INTEGER,
  size INTEGER,
  active BOOLEAN NOT NULL DEFAULT 'f'
);

-- Your SQL goes here
CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  chat_id INTEGER NOT NULL,
  message_type INTEGER NOT NULL,
  task VARCHAR(15) NOT NULL,
  content VARCHAR(40) NOT NULL,
  fullfilled BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE voice_permissions (
  id SERIAL PRIMARY KEY,
  voice_id SERIAL,
  owner_chat_id INTEGER NOT NULL,
  voice_file_id VARCHAR(40) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  FOREIGN KEY ("voice_id") REFERENCES voices(id)
);