-- Your SQL goes here
-- Sysinfo table
CREATE TABLE sysinfo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    cpu_usage REAL NOT NULL,
    ram_usage REAL NOT NULL,
    total_ram REAL NOT NULL,
    free_ram REAL NOT NULL,
    used_swap REAL NOT NULL
    -- user_id INTEGER NOT NULL,
    -- FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);