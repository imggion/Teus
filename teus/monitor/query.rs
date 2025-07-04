use crate::monitor::schema::{DiskInfo, SysInfo};
use diesel::prelude::*;
use diesel::result::Error;

/// Fetches the latest SysInfo record along with its associated DiskInfo records.
pub fn get_latest_sysinfo_with_disks(
    conn: &mut SqliteConnection,
) -> Result<Option<(SysInfo, Vec<DiskInfo>)>, Error> {
    use crate::schema::sysinfo::dsl::*;

    // 1. Get the latest SysInfo record
    let latest_sysinfo_option = sysinfo
        .order(id.desc()) // Order by ID descending to get the latest
        .select(SysInfo::as_select())
        .first::<SysInfo>(conn)
        .optional()?;

    match latest_sysinfo_option {
        Some(latest_sysinfo) => {
            // 2. If a SysInfo record was found, find all DiskInfo records that match
            use crate::schema::diskinfo::dsl::*;

            let disks = diskinfo
                .filter(sysinfo_id.eq(latest_sysinfo.id.unwrap()))
                .select(DiskInfo::as_select())
                .load::<DiskInfo>(conn)?; // Load all associated disks

            Ok(Some((latest_sysinfo, disks)))
        }
        None => {
            // No SysInfo records found
            Ok(None)
        }
    }
}
