-- Your SQL goes here
CREATE TABLE items (
  id TEXT NOT NULL PRIMARY KEY,
  title TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);
