-- Add up migration script here
CREATE TABLE
  items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    username TEXT,
    password TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
  );

CREATE TABLE
  websites (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id INTEGER NOT NULL,
    url TEXT,
    FOREIGN KEY (item_id) references items (id)
  );

CREATE TABLE
  sections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id INTEGER NOT NULL,
    name TEXT,
    FOREIGN KEY (item_id) references items (id)
  );

CREATE TABLE
  columns (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id INTEGER,
    section_id INTEGER,
    name TEXT,
    value TEXT,
    value_type INTEGER NOT NULL,
    FOREIGN KEY (item_id) references items (id),
    FOREIGN KEY (section_id) references sections (id)
  );

CREATE TABLE
  tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
  );

CREATE TABLE
  item_tag (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (item_id) REFERENCES items (id),
    FOREIGN KEY (tag_id) REFERENCES tags (id)
  );