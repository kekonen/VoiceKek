-- Your SQL goes here
CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  chat_id INTEGER NOT NULL,
  message_type INTEGER NOT NULL,
  task VARCHAR(15) NOT NULL,
  content VARCHAR(40) NOT NULL,
  fullfilled BOOLEAN NOT NULL DEFAULT 'f'
)