use crate::{
    config::{schema::TeusConfig, types::Config},
    monitor::storage::Storage,
    webserver::auth::{middleware::Claims, schema::User},
};
use actix_web::{HttpResponse, Responder, post, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

/// Request structure for user authentication login endpoint.
///
/// This structure represents the data required for a user to authenticate
/// with the Teus system. It contains the credentials that will be validated
/// against the stored user data in the database.
///
/// # Security Considerations
///
/// - The password field should be transmitted over HTTPS in production
/// - Passwords are never logged or stored in plain text
/// - Consider implementing rate limiting to prevent brute force attacks
///
/// # Examples
///
/// JSON request body:
/// ```json
/// {
///   "username": "admin",
///   "password": "secure_password"
/// }
/// ```
///
/// # API Endpoint
///
/// Used with `POST /auth/login`
#[derive(Deserialize)]
pub struct LoginRequest {
    /// The user's login identifier.
    ///
    /// This should be a unique username that exists in the system.
    /// Case-sensitive matching is performed against stored usernames.
    username: String,
    
    /// The user's password in plain text.
    ///
    /// This will be verified against the stored password hash using
    /// Argon2 password hashing. The plain text password is never
    /// stored or logged by the system.
    password: String,
}

/// Request structure for user registration endpoint.
///
/// This structure represents the data required to create a new user
/// account in the Teus system. The provided password will be securely
/// hashed using Argon2 before storage.
///
/// # Validation Requirements
///
/// - Username must be unique in the system
/// - Password should meet minimum security requirements (implemented at application level)
/// - Both fields are required and cannot be empty
///
/// # Security Features
///
/// - Passwords are hashed with Argon2 and a unique salt
/// - Plain text passwords are never stored
/// - Username uniqueness is enforced at the database level
///
/// # Examples
///
/// JSON request body:
/// ```json
/// {
///   "username": "newuser",
///   "password": "secure_password123"
/// }
/// ```
///
/// # API Endpoint
///
/// Used with `POST /auth/signup`
#[derive(Deserialize)]
pub struct SignupRequest {
    /// The desired username for the new account.
    ///
    /// Must be unique across all users in the system. If a user
    /// with this username already exists, the registration will fail
    /// with a 409 Conflict status.
    username: String,
    
    /// The password for the new account in plain text.
    ///
    /// This will be securely hashed using Argon2 with a unique salt
    /// before being stored in the database. The plain text password
    /// is never persisted.
    password: String,
}

/// Response structure for successful authentication operations.
///
/// This structure contains the JWT tokens issued after successful login
/// or token refresh operations. It provides both access and refresh tokens
/// following OAuth 2.0-style token patterns.
///
/// # Token Types
///
/// - **Access Token**: Short-lived token for API authentication (default: configurable hours)
/// - **Refresh Token**: Long-lived token for obtaining new access tokens (default: 7 days)
///
/// # Security Notes
///
/// - Access tokens should be stored securely on the client (e.g., memory, secure storage)
/// - Refresh tokens should be stored even more securely and used only for token renewal
/// - Both tokens are JWTs signed with the server's secret key
///
/// # Examples
///
/// JSON response:
/// ```json
/// {
///   "access": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
///   "refresh": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
///   "expires_in": 3600
/// }
/// ```
#[derive(Serialize)]
pub struct TokenResponse {
    /// JWT access token for API authentication.
    ///
    /// This token should be included in the Authorization header
    /// of subsequent API requests as "Bearer {token}". It has a
    /// shorter expiration time for security.
    access: String,
    
    /// JWT refresh token for obtaining new access tokens.
    ///
    /// This token can be used to obtain a new access token when
    /// the current one expires, without requiring the user to
    /// log in again. It has a longer expiration time.
    refresh: String,
    
    /// Time until the access token expires, in seconds.
    ///
    /// Clients should use this value to determine when to refresh
    /// the access token using the refresh token. This is typically
    /// calculated as expiration_hours * 3600.
    expires_in: i64,
}

/// Configuration structure for JWT token generation and validation.
///
/// This structure holds the configuration parameters needed for
/// JSON Web Token operations in the authentication system. It's
/// typically initialized once at application startup.
///
/// # Security Considerations
///
/// - The secret should be cryptographically secure and sufficiently long
/// - The secret should be different for each environment (dev, test, prod)
/// - Consider rotating secrets periodically in production environments
/// - Never log or expose the secret in error messages
///
/// # Examples
///
/// ```rust
/// use teus::webserver::auth::handlers::JwtConfig;
///
/// let jwt_config = JwtConfig {
///     secret: "your-256-bit-secret-key".to_string(),
///     expiration_hours: 1, // 1 hour for access tokens
/// };
/// ```
pub struct JwtConfig {
    /// Secret key used for signing and verifying JWT tokens.
    ///
    /// This should be a cryptographically secure random string,
    /// at least 256 bits (32 characters) long. The same secret
    /// must be used for both token generation and validation.
    pub secret: String,
    
    /// Number of hours until access tokens expire.
    ///
    /// Shorter expiration times improve security by reducing the
    /// window of opportunity if a token is compromised, but may
    /// require more frequent token refreshes. Typical values
    /// range from 1-24 hours.
    pub expiration_hours: i64,
}

/// Generic response structure for simple API messages.
///
/// This structure is used for API endpoints that need to return
/// a simple text message, such as error responses or confirmation
/// messages. It provides a consistent format for client applications.
///
/// # Usage Patterns
///
/// - Error messages (e.g., "Invalid credentials", "User not found")
/// - Success confirmations (e.g., "Operation completed")
/// - Validation errors (e.g., "Username already exists")
///
/// # Examples
///
/// JSON response:
/// ```json
/// {
///   "message": "Invalid credentials"
/// }
/// ```
///
/// # TODO
///
/// Consider moving this to a separate common response module
/// to be shared across different handlers and avoid duplication.
#[derive(Serialize)]
struct GenericResponse {
    /// The message content to be returned to the client.
    ///
    /// Should be user-friendly and provide clear information
    /// about the result of the operation or the nature of any error.
    message: String,
}

/// Response structure for successful user registration.
///
/// This structure contains the essential information about a newly
/// created user account, returned after successful signup operations.
/// It excludes sensitive information like passwords and salts.
///
/// # Security Notes
///
/// - Only non-sensitive user information is included
/// - Password hashes and salts are never included in responses
/// - The user ID can be used for subsequent API operations
///
/// # Examples
///
/// JSON response:
/// ```json
/// {
///   "id": 1,
///   "username": "newuser"
/// }
/// ```
///
/// # API Endpoint
///
/// Returned by `POST /auth/signup` on successful user creation
#[derive(Serialize)]
struct NewUserResponse {
    /// The database-generated unique identifier for the new user.
    ///
    /// This ID is used internally by the system to reference
    /// the user in database relations and API operations.
    id: i32,
    
    /// The username of the newly created account.
    ///
    /// Confirms the username that was successfully registered,
    /// useful for client-side confirmation and user feedback.
    username: String,
}

// Verify the password against the hash
fn is_same_password(password_hash: &str, clear_pass: &str, _salt: &str) -> bool {
    // Try to parse the password hash
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    // Verify the password against the hash
    Argon2::default()
        .verify_password(clear_pass.as_bytes(), &parsed_hash)
        .is_ok()
}

// Handler di login
#[post("/login")]
pub async fn login(
    login_data: web::Json<LoginRequest>,
    jwt_config: web::Data<JwtConfig>,
    config: actix_web::web::Data<Config>,
) -> impl Responder {
    let storage = Storage::new(&config.database.path).unwrap();
    let mut conn = storage.diesel_conn.lock().unwrap();
    let user = User::find_by_username(&mut *conn, &login_data.username).unwrap();
    let user_id: i32;

    match user {
        Some(user) => {
            println!("User found: {:?}", user);
            let is_password_correct =
                is_same_password(&user.password, &login_data.password, &user.salt);

            if !is_password_correct {
                let response = GenericResponse {
                    message: "Invalid Credentials".to_string(),
                };
                return HttpResponse::Unauthorized().json(response);
            }

            user_id = user.id.unwrap();
        }
        None => {
            println!("User not found");
            let response = GenericResponse {
                message: "Invalid Credentials".to_string(),
            };
            return HttpResponse::Unauthorized().json(response);
        }
    }

    // Calcola la scadenza per access token
    let access_expiration = Utc::now()
        .checked_add_signed(Duration::hours(jwt_config.expiration_hours))
        .expect("Valid timestamp")
        .timestamp() as usize;

    // Calcola la scadenza per refresh token (più lunga, ad esempio 7 giorni)
    let refresh_expiration = Utc::now()
        .checked_add_signed(Duration::hours(24 * 7)) // 7 days
        .expect("Valid timestamp")
        .timestamp() as usize;

    // Crea i claims per access token
    let access_claims = Claims {
        sub: login_data.username.clone(),
        exp: access_expiration,
        iat: Utc::now().timestamp() as usize,
        id: user_id,
    };

    // Crea i claims per refresh token
    let refresh_claims = Claims {
        sub: login_data.username.clone(),
        exp: refresh_expiration,
        iat: Utc::now().timestamp() as usize,
        id: user_id,
    };

    // Genera l'access token
    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(jwt_config.secret.as_bytes()),
    )
    .unwrap();

    // Genera il refresh token
    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(jwt_config.secret.as_bytes()),
    )
    .unwrap();

    // Restituisci i token
    let response = TokenResponse {
        access: access_token,
        refresh: refresh_token,
        expires_in: jwt_config.expiration_hours * 3600,
    };

    HttpResponse::Ok().json(response)
}

#[post("/signup")]
pub async fn signup(
    signup_data: web::Json<SignupRequest>,
    config: actix_web::web::Data<Config>,
) -> impl Responder {
    let storage = Storage::new(&config.database.path).unwrap();
    let mut conn = storage.diesel_conn.lock().unwrap();

    let existing_user = User::find_by_username(&mut *conn, &signup_data.username).unwrap();
    if existing_user.is_some() {
        let response = GenericResponse {
            message: "Username already exists".to_string(),
        };
        return HttpResponse::Conflict().json(response);
    }
    let user = User::create(&mut *conn, &signup_data.username, &signup_data.password).unwrap();
    TeusConfig::set_first_visit(&mut *conn, false).unwrap();

    // Create a response without the sensitive data
    let user_response = NewUserResponse {
        id: user.id.unwrap(),
        username: user.username,
    };

    HttpResponse::Created().json(user_response)
}
