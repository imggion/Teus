//! User authentication schema structures.
//!
//! This module defines the database schema structure for user accounts
//! in the Teus system. It handles user authentication data including
//! secure password storage with Argon2 hashing and salt generation.

use diesel::prelude::*;
use serde::Serialize;

/// Database schema structure for user accounts.
///
/// This structure represents user authentication data stored in the SQLite
/// database. It includes all necessary fields for secure user management
/// including password hashing and salt storage.
///
/// # Security Features
///
/// - Passwords are stored as Argon2 hashes, never in plain text
/// - Each password has a unique salt to prevent rainbow table attacks
/// - Username uniqueness is enforced at the database level
/// - User IDs are auto-generated and used for session management
///
/// # Database Operations
///
/// This structure supports both insertion (user creation) and querying
/// (user lookup) operations through Diesel ORM. The `Insertable` trait
/// allows new user creation, while `Queryable` enables data retrieval.
///
/// # JSON Serialization
///
/// The structure can be serialized to JSON for API responses, but note
/// that password hashes and salts should typically be excluded from
/// client-facing responses for security reasons.
///
/// # Examples
///
/// Creating a new user (handled by implementation methods):
/// ```rust
/// use teus::webserver::auth::schema::User;
/// // User creation is handled by User::create() method with proper hashing
/// ```
///
/// # Related Modules
///
/// - `mutation.rs`: Contains `User::create()` for user registration
/// - `query.rs`: Contains `User::find_by_username()` for authentication
/// - `handlers.rs`: Uses this structure in login/signup endpoints
#[derive(Insertable, Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    /// Database-generated unique identifier for the user.
    ///
    /// This is the primary key and is automatically assigned when
    /// a new user is created. Used for session management and
    /// referencing the user in JWT tokens and other operations.
    ///
    /// # Database Behavior
    ///
    /// - `None` for new users before insertion
    /// - `Some(id)` for existing users retrieved from database
    pub id: Option<i32>,

    /// Unique username for the user account.
    ///
    /// This serves as the primary login identifier and must be unique
    /// across all users in the system. Username matching is case-sensitive.
    ///
    /// # Constraints
    ///
    /// - Must be unique (enforced by database)
    /// - Cannot be empty or null
    /// - Used for login authentication
    ///
    /// # Security Notes
    ///
    /// - Safe to include in API responses and logs
    /// - Used as the JWT token subject (`sub` claim)
    pub username: String,

    /// Argon2 password hash for secure authentication.
    ///
    /// This field stores the hashed representation of the user's password
    /// using the Argon2 algorithm with a unique salt. The original password
    /// is never stored in the database.
    ///
    /// # Security Features
    ///
    /// - Uses Argon2id variant for maximum security
    /// - Includes salt, iteration count, and memory parameters
    /// - Resistant to rainbow table and brute force attacks
    /// - Format follows PHC string format for compatibility
    ///
    /// # Important Notes
    ///
    /// - **NEVER** include this field in API responses
    /// - **NEVER** log this field value
    /// - Only used for password verification during login
    /// - Updated only when user changes their password
    pub password: String,

    /// Cryptographic salt used for password hashing.
    ///
    /// This field stores the unique salt that was used when hashing
    /// the user's password. Each user has a different salt to prevent
    /// rainbow table attacks and ensure password hash uniqueness.
    ///
    /// # Security Properties
    ///
    /// - Cryptographically random and unique per user
    /// - Generated using secure random number generation
    /// - Combined with password before hashing
    /// - Stored separately but used during verification
    ///
    /// # Implementation Details
    ///
    /// - Generated using `SaltString::generate()` from the `argon2` crate
    /// - Base64-encoded string format for database storage
    /// - Required for Argon2 password verification process
    ///
    /// # Security Warning
    ///
    /// Like the password hash, this field should **NEVER** be included
    /// in API responses or logged, as it could potentially aid in
    /// password cracking attempts.
    pub salt: String,
}
