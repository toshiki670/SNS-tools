-- Add down migration script here
PRAGMA foreign_keys = false;

DROP TABLE items;

DROP TABLE sections;

DROP TABLE columns;

DROP TABLE tags;

DROP TABLE item_tag;