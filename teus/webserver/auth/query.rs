use super::schema::User;
use diesel::{prelude::*, result::Error};

impl User {
    pub fn find_by_username(
        conn: &mut SqliteConnection,
        username_q: &str,
    ) -> Result<Option<User>, Error> {
        use crate::schema::user::dsl::*;

        user.filter(username.eq(username_q))
            .select(User::as_select())
            .first(conn)
            .optional()
    }
}
