// src/monitor/mutation.rs
use crate::schema::{SchemaDiskInfo, SchemaSysInfo};
use diesel::prelude::*;
use diesel::result::Error;

/// Inserts system information into the database and returns the ID of the new record.
pub fn insert_sysinfo(
    conn: &mut SqliteConnection,
    new_sys_info: &SchemaSysInfo,
) -> Result<i32, Error> {
    use teus_schema::schema::sysinfo::dsl::*;

    diesel::insert_into(sysinfo)
        .values(new_sys_info)
        // SQLite doesn't directly support RETURNING id easily with diesel's insert helper
        // So we insert, then query the last inserted row's id.
        // This assumes single-threaded insertion or other mechanisms to prevent race conditions.
        .execute(conn)?;

    // Retrieve the ID of the last inserted row for SQLite
    let inserted_id = sysinfo
        .select(id)
        .order(id.desc())
        .first::<Option<i32>>(conn)?
        .ok_or(Error::NotFound)?; // Should exist if insert succeeded

    Ok(inserted_id)
}

#[allow(dead_code)]
/// Inserts disk information into the database.
pub fn insert_diskinfo(
    conn: &mut SqliteConnection,
    new_disk_info: &SchemaDiskInfo,
) -> Result<usize, Error> {
    use teus_schema::schema::diskinfo::dsl::*;

    diesel::insert_into(diskinfo)
        .values(new_disk_info)
        .execute(conn) // Returns the number of affected rows
}

/// Inserts multiple disk info entries efficiently.
pub fn insert_multiple_diskinfo(
    conn: &mut SqliteConnection,
    disk_infos: &[SchemaDiskInfo],
) -> Result<usize, Error> {
    use teus_schema::schema::diskinfo::dsl::*;

    diesel::insert_into(diskinfo)
        .values(disk_infos)
        .execute(conn)
}
