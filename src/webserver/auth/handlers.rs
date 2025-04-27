use crate::{
    config::types::Config,
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

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct SignupRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    access: String,
    refresh: String,
    expires_in: i64,
}

pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

#[derive(Serialize)]
// TODO: Move this to a separate file for every kind of response
struct GenericResponse {
    message: String,
}

#[derive(Serialize)]
struct NewUserResponse {
    id: i32,
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
    let mut user_id: i32 = 0;

    match user {
        Some(user) => {
            println!("User found: {:?}", user);
            let is_password_correct =
                is_same_password(&user.password, &login_data.password, &user.salt);

            if !is_password_correct {
                return HttpResponse::Unauthorized().json("Invalid credentials");
            }

            user_id = user.id.unwrap();
        }
        None => {
            println!("User not found");
            return HttpResponse::Unauthorized().json("Invalid credentials");
        }
    }

    // Calcola la scadenza
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(jwt_config.expiration_hours))
        .expect("Valid timestamp")
        .timestamp() as usize;

    // Crea i claims
    let claims = Claims {
        sub: login_data.username.clone(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
        id: user_id,
    };

    // Genera il token
    let access_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_config.secret.as_bytes()),
    )
    .unwrap();

    // Restituisci il token
    let response = TokenResponse {
        access: access_token,
        refresh: "Not implemented".to_string(),
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

    // Create a response without the sensitive data
    let user_response = NewUserResponse {
        id: user.id.unwrap(),
        username: user.username,
    };

    HttpResponse::Created().json(user_response)
}
