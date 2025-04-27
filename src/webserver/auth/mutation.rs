use super::schema::User;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use diesel::result::Error;
use diesel::{RunQueryDsl, SqliteConnection};

impl User {
    pub fn create(
        conn: &mut SqliteConnection,
        username: &str,
        password: &str,
    ) -> Result<User, Error> {
        use crate::schema::user;

        // Get a random salt for the password
        let salt = SaltString::generate(&mut OsRng);
        
        // Create an Argon2 instance with default parameters
        let argon2 = Argon2::default();
        
        // Generate the password hash as a string
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let new_user = User {
            id: None,
            username: username.to_string(),
            password: password_hash,
            salt: salt.as_str().to_owned(),
        };

        diesel::insert_into(user::table)
            .values(&new_user)
            .get_result(conn)
    }
}
