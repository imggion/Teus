use super::schema::TeusConfig;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{RunQueryDsl, SqliteConnection};

impl TeusConfig {
    pub fn set_first_visit(conn: &mut SqliteConnection, is_first_visit: bool) -> Result<(), Error> {
        use crate::schema::config::dsl::*;

        // Get the first config record
        let teus_config = config
            .select(TeusConfig::as_select())
            .first::<TeusConfig>(conn)
            .unwrap();

        // Safely unwrap the ID or handle None case
        let config_id = match teus_config.id {
            Some(cid) => cid,
            None => return Err(Error::NotFound),
        };

        // Update the record
        diesel::update(config.filter(id.eq(config_id)))
            .set(first_visit.eq(is_first_visit))
            .execute(conn)
            .unwrap();

        println!(
            "Updated first_visit to {:?} for TeusConfig with id {}",
            is_first_visit, config_id
        );

        Ok(())
    }
}
