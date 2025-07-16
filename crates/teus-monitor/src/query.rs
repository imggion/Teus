use crate::schema::{DiskInfo, SysInfo};
use diesel::prelude::*;
use diesel::result::Error;

/// Fetches the latest SysInfo record along with its associated DiskInfo records.
pub fn get_latest_sysinfo_with_disks(
    conn: &mut SqliteConnection,
) -> Result<Option<(SysInfo, Vec<DiskInfo>)>, Error> {
    use teus_schema::schema::sysinfo::dsl::*;

    let latest_sysinfo_option = sysinfo
        .order(id.desc()) // Order by ID descending to get the latest
        .select(SysInfo::as_select())
        .first::<SysInfo>(conn)
        .optional()?;

    match latest_sysinfo_option {
        Some(latest_sysinfo) => {
            /* import here because the ID is ambiguos */
            use teus_schema::schema::diskinfo::dsl::*;

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
