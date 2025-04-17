use super::schema::TeusConfig;
use diesel::prelude::*;
use diesel::result::Error;

pub fn get_teus_server_config(conn: &mut SqliteConnection) -> Result<Option<(TeusConfig)>, Error> {
    use crate::schema::config::dsl::*;

    let latest_teusconfig_option = config
        .select(TeusConfig::as_select())
        .first::<TeusConfig>(conn)
        .optional()?;

    match latest_teusconfig_option {
        Some(latest_teusconfig) => Ok(Some(latest_teusconfig)),
        None => {
            // No SysInfo records found
            Ok(None)
        }
    }
}
