use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, params};
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::monitor::sys::{DiskInfo, SysInfo};

#[derive(Clone)]
pub struct Storage {
    pub conn: Arc<Connection>,
}

mod storage_utils {
    use std::{fs, io, path::Path};

    pub fn ensure_directory_exists(path: &str) -> io::Result<()> {
        let dir_path = Path::new(path);
        if !dir_path.exists() {
            fs::create_dir_all(dir_path)?;
            println!("Directory '{}' created.", path);
        }
        Ok(())
    }
}

impl Storage {
    pub fn new(db_path: &str) -> rusqlite::Result<Self> {
        // Ottieni il path della directory padre
        if let Some(parent) = Path::new(db_path).parent() {
            if let Some(parent_str) = parent.to_str() {
                // Controlla e crea la directory se non esiste
                storage_utils::ensure_directory_exists(parent_str)
                    .expect("Failed to create parent directory");
            }
        }

        // Apre o crea il file SQLite nel path specificato
        let conn = Connection::open(db_path)?;

        // Enable WAL mode for better concurrency
        conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;

        // Set a busy timeout to wait instead of immediately failing
        conn.busy_timeout(Duration::from_secs(5))?;

        Ok(Self {
            conn: Arc::new(conn),
        })
    }

    /// Initialize the database, by default it creates the tables if they don't exist.
    /// Otherwise it does nothing.
    pub fn init_db(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sysinfo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            cpu_usage REAL NOT NULL,
            ram_usage REAL NOT NULL,
            total_ram REAL NOT NULL,
            free_ram REAL NOT NULL,
            used_swap REAL NOT NULL
        )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS diskinfo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sysinfo_id INTEGER NOT NULL,
            filesystem TEXT NOT NULL,
            size INTEGER NOT NULL,
            used INTEGER NOT NULL,
            available INTEGER NOT NULL,
            used_percentage INTEGER NOT NULL,
            mounted_path TEXT NOT NULL,
            FOREIGN KEY (sysinfo_id) REFERENCES sysinfo(id)
        )",
            [],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn system_time_to_iso8601(&self, time: SystemTime) -> String {
        let datetime: DateTime<Utc> = time.into();
        datetime.to_rfc3339()
    }

    pub fn insert_sysinfo(&self, conn: &Connection, sysinfo: &SysInfo) -> Result<i64> {
        // Use timestamp directly as it's already a String
        conn.execute(
            "INSERT INTO sysinfo (timestamp, cpu_usage, ram_usage, total_ram, free_ram, used_swap)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                sysinfo.timestamp,
                sysinfo.cpu_usage,
                sysinfo.ram_usage,
                sysinfo.total_ram,
                sysinfo.free_ram,
                sysinfo.used_swap
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_latest_sysinfo(&self, conn: &Connection) -> Result<Option<SysInfo>> {
        // Recupera l'ultimo record dalla tabella sysinfo
        let mut stmt = conn.prepare(
            "SELECT s.id, s.timestamp, s.cpu_usage, s.ram_usage, s.total_ram, s.free_ram, s.used_swap,
                    d.filesystem, d.size, d.used, d.available, d.used_percentage, d.mounted_path
             FROM sysinfo s
             LEFT JOIN diskinfo d ON s.id = d.sysinfo_id
             WHERE s.id = (SELECT id FROM sysinfo ORDER BY id DESC LIMIT 1)"
        )?;

        let mut rows = stmt.query([])?;
        let mut sysinfo: Option<SysInfo> = None;

        while let Some(row) = rows.next()? {
            // Se sysinfo non è ancora stato creato, lo creiamo usando i dati della prima riga
            if sysinfo.is_none() {
                sysinfo = Some(SysInfo {
                    id: row.get("id")?,
                    timestamp: row.get("timestamp")?,
                    cpu_usage: row.get("cpu_usage")?,
                    ram_usage: row.get("ram_usage")?,
                    total_ram: row.get("total_ram")?,
                    free_ram: row.get("free_ram")?,
                    used_swap: row.get("used_swap")?,
                    disks: Vec::new(),
                });
            }
            // Recuperiamo i dati relativi al disco: se filesystem è NULL, significa che non esiste un record in diskinfo.
            let filesystem: Option<String> = row.get("filesystem")?;
            if let Some(fs) = filesystem {
                let disk = DiskInfo {
                    filesystem: fs,
                    size: row.get("size")?,
                    used: row.get("used")?,
                    available: row.get("available")?,
                    used_percentage: row.get("used_percentage")?,
                    mounted_path: row.get("mounted_path")?,
                };
                if let Some(ref mut info) = sysinfo {
                    info.disks.push(disk);
                }
            }
        }

        Ok(sysinfo)
    }

    pub fn insert_diskinfo(
        &self,
        conn: &Connection,
        sysinfo_id: i64,
        disk: &DiskInfo,
    ) -> Result<()> {
        conn.execute(
        "INSERT INTO diskinfo (sysinfo_id, filesystem, size, used, available, used_percentage, mounted_path)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
         params![
             sysinfo_id,
             disk.filesystem,
             disk.size as i64,
             disk.used as i64,
             disk.available as i64,
             disk.used_percentage as i64,
             disk.mounted_path
         ],
    )?;
        Ok(())
    }
}
