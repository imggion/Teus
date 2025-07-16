use super::schema::TeusConfig;
use diesel::prelude::*;
use diesel::result::Error;

impl TeusConfig {
    pub fn is_first_visit(conn: &mut SqliteConnection) -> Result<bool, Error> {
        // use crate::schema::config::dsl::*;
        use teus_schema::schema::config::dsl::*;

        config.select(first_visit).first(conn)
    }

    pub fn get_teus_server_config(
        conn: &mut SqliteConnection,
    ) -> Result<Option<TeusConfig>, Error> {
        use teus_schema::schema::config::dsl::*;

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
}
