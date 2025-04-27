use diesel::{Connection as ConnectionDiesel, SqliteConnection};
use rusqlite::{Connection, Result};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct Storage {
    // pub conn: Arc<Connection>, // @Info: old Arc reference to don't break the code
    pub diesel_conn: Arc<Mutex<SqliteConnection>>, // @Info: use to test diesel for now
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

// TODO: Migrate Connection -> SqliteConnection
impl Storage {
    pub fn new(db_path: &str) -> rusqlite::Result<Self> {
        if let Some(parent) = Path::new(db_path).parent() {
            if let Some(parent_str) = parent.to_str() {
                storage_utils::ensure_directory_exists(parent_str)
                    .expect("Failed to create parent directory");
            }
        }

        let conn = Connection::open(db_path)?;
        let conn_new = SqliteConnection::establish(&db_path)
            .unwrap_or_else(|e| panic!("Failed to connect, error: {}", e));

        // Enable WAL mode for better concurrency
        conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;

        // Set a busy timeout to wait instead of immediately failing
        conn.busy_timeout(Duration::from_secs(5))?;

        Ok(Self {
            diesel_conn: Arc::new(Mutex::new(conn_new)),
        })
    }

    /// Initialize the database, by default it creates the tables if they don't exist.
    /// Otherwise it does nothing.
    #[deprecated = "Now uses Diesel migrations instead"]
    pub fn _init_db(&self, conn: &Connection) -> Result<()> {
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
}
