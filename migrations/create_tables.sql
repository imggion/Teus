CREATE TABLE IF NOT EXISTS sysinfo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    cpu_usage REAL NOT NULL,
    ram_usage REAL NOT NULL,
    total_ram REAL NOT NULL,
    free_ram REAL NOT NULL,
    used_swap REAL NOT NULL
);

CREATE TABLE IF NOT EXISTS diskinfo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sysinfo_id INTEGER NOT NULL,
    filesystem TEXT NOT NULL,
    size INTEGER NOT NULL,
    used INTEGER NOT NULL,
    available INTEGER NOT NULL,
    used_percentage INTEGER NOT NULL,
    mounted_path TEXT NOT NULL,
    FOREIGN KEY (sysinfo_id) REFERENCES sysinfo(id)
);