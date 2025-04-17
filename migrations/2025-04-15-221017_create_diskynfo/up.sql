-- Your SQL goes here
-- Diskinfo table
CREATE TABLE diskinfo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sysinfo_id INTEGER NOT NULL,
    filesystem TEXT NOT NULL,
    size INTEGER NOT NULL,
    used INTEGER NOT NULL,
    available INTEGER NOT NULL,
    used_percentage INTEGER NOT NULL,
    mounted_path TEXT NOT NULL,
    FOREIGN KEY (sysinfo_id) REFERENCES sysinfo(id) ON DELETE CASCADE
);