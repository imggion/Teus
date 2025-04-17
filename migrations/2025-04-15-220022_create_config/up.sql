-- Your SQL goes here
-- Config table (only one row expected)
CREATE TABLE config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    first_visit BOOLEAN NOT NULL DEFAULT 1
);

INSERT INTO config (first_visit) VALUES (0);
