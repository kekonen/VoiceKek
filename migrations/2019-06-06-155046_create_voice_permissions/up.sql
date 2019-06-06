-- Your SQL goes here
CREATE TABLE voice_permissions (
  id SERIAL PRIMARY KEY,
  owner_chat_id INTEGER NOT NULL,
  voice_file_id VARCHAR(31) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
)