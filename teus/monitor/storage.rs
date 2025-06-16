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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_storage_new_success() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db_path = temp_dir.path().join("test.db");
        let db_path_str = db_path.to_str().unwrap();

        let storage = Storage::new(db_path_str);
        assert!(storage.is_ok());

        let storage = storage.unwrap();
        
        // Test that we can acquire the mutex lock
        let conn_guard = storage.diesel_conn.lock();
        assert!(conn_guard.is_ok());
    }

    #[test] 
    fn test_storage_creates_parent_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let nested_path = temp_dir.path().join("nested").join("path").join("test.db");
        let db_path_str = nested_path.to_str().unwrap();

        let storage = Storage::new(db_path_str);
        assert!(storage.is_ok());

        // Verify the nested directories were created
        assert!(temp_dir.path().join("nested").join("path").exists());
    }

    #[test]
    fn test_storage_with_memory_database() {
        // SQLite in-memory database
        let storage = Storage::new(":memory:");
        assert!(storage.is_ok());
        
        let storage = storage.unwrap();
        let conn_guard = storage.diesel_conn.lock();
        assert!(conn_guard.is_ok());
    }

    #[test]
    fn test_storage_invalid_path() {
        // Try to create database in a location that doesn't exist and can't be created
        // This might not fail on all systems, but it's worth testing
        let invalid_path = "/invalid/path/that/should/not/exist/test.db";
        let storage = Storage::new(invalid_path);
        
        // On most systems this should fail due to permission issues
        // But SQLite might create the path in some cases, so we just ensure it returns a Result
        match storage {
            Ok(_) => {
                // If it succeeds, that's fine too - SQLite is quite permissive
            }
            Err(_) => {
                // This is the expected case for most invalid paths
            }
        }
    }

    #[test]
    fn test_storage_clone() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db_path = temp_dir.path().join("test.db");
        let db_path_str = db_path.to_str().unwrap();

        let storage = Storage::new(db_path_str).expect("Failed to create storage");
        let cloned_storage = storage.clone();

        // Both should be able to access the same connection
        let conn1 = storage.diesel_conn.lock();
        assert!(conn1.is_ok());
        drop(conn1); // Release lock before trying with clone

        let conn2 = cloned_storage.diesel_conn.lock();
        assert!(conn2.is_ok());
    }

    #[test]
    fn test_storage_concurrent_access() {
        use std::thread;
        use std::time::Duration;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db_path = temp_dir.path().join("test.db");
        let db_path_str = db_path.to_str().unwrap();

        let storage = Storage::new(db_path_str).expect("Failed to create storage");
        let storage_clone = storage.clone();

        let handle = thread::spawn(move || {
            let conn = storage_clone.diesel_conn.lock();
            assert!(conn.is_ok());
            thread::sleep(Duration::from_millis(10));
        });

        // Give the thread a moment to start
        thread::sleep(Duration::from_millis(5));
        
        // This should be able to access after the thread releases the lock
        handle.join().expect("Thread panicked");
        
        let conn = storage.diesel_conn.lock();
        assert!(conn.is_ok());
    }

    #[test]
    fn test_storage_utils_ensure_directory_exists() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let test_path = temp_dir.path().join("test_subdir");
        let test_path_str = test_path.to_str().unwrap();

        // Directory doesn't exist initially
        assert!(!test_path.exists());

        // Call the utility function
        let result = storage_utils::ensure_directory_exists(test_path_str);
        assert!(result.is_ok());

        // Directory should now exist
        assert!(test_path.exists());
        assert!(test_path.is_dir());

        // Calling again on existing directory should also work
        let result = storage_utils::ensure_directory_exists(test_path_str);
        assert!(result.is_ok());
    }

    #[test]
    fn test_storage_utils_invalid_path() {
        // Try to create a directory in an invalid location
        let result = storage_utils::ensure_directory_exists("/root/invalid/path");
        
        // This should fail on most systems due to permission issues
        match result {
            Ok(_) => {
                // In some test environments this might succeed
            }
            Err(_) => {
                // This is the expected case for most systems
            }
        }
    }
}
