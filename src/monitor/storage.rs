use diesel::{Connection as ConnectionDiesel, SqliteConnection};
use diesel::connection::SimpleConnection; // Added
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::error::Error; // Added for Box<dyn Error>

// Removed: use rusqlite::{Connection, Result};
// Removed: use std::time::Duration;

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
            // Consider using log crate for messages instead of println!
            // println!("Directory '{}' created.", path); 
        }
        Ok(())
    }
}

// TODO: Migrate Connection -> SqliteConnection // This TODO can be removed after this refactor
impl Storage {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn Error>> { // Changed return type
        if let Some(parent) = Path::new(db_path).parent() {
            if let Some(parent_str) = parent.to_str() {
                storage_utils::ensure_directory_exists(parent_str)?; // Changed from expect
            }
        }

        // Removed rusqlite connection logic

        let mut conn_new = SqliteConnection::establish(&db_path)?; // Changed from unwrap_or_else

        // Apply PRAGMAs to Diesel connection
        // Note: busy_timeout is set in milliseconds for SQLite PRAGMA
        conn_new.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL; PRAGMA busy_timeout = 5000;")?;

        Ok(Self {
            diesel_conn: Arc::new(Mutex::new(conn_new)),
        })
    }

    // Removed _init_db method
}
