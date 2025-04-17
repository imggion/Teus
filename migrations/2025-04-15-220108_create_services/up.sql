-- Your SQL goes here
-- Services table
CREATE TABLE services (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    link TEXT NOT NULL,
    icon TEXT,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);