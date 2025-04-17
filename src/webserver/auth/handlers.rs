use crate::{
    config::types::Config,
    monitor::storage::Storage,
    webserver::auth::{middleware::Claims, schema::User},
};
use actix_web::{HttpResponse, Responder, post, web};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

// Struttura per i dati di login
#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

// Risposta di login con token
#[derive(Serialize)]
pub struct TokenResponse {
    access: String,
    refresh: String,
    expires_in: i64,
}

// Configurazione JWT
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

// Handler di login
#[post("/login")]
pub async fn login(
    login_data: web::Json<LoginRequest>,
    jwt_config: web::Data<JwtConfig>,
    config: actix_web::web::Data<Config>,
) -> impl Responder {
    // TODO: Implement user verification from database

    /* ====================== TEST DIESEL AREA ======================== */
    let mut storage = Storage::new(&config.database.path).unwrap();
    // Assuming diesel_conn is an Arc<SqliteConnection>
    let mut conn = storage.diesel_conn.lock().unwrap();
    let some_user_test = User::find_by_username(&mut *conn, &login_data.username).unwrap();
    let mut user_id: i32 = 0;

    match some_user_test {
        Some(user) => {
            println!("User found: {:?}", user);
            if login_data.password != user.password {
                return HttpResponse::Unauthorized().json("Invalid credentials");
            }

            user_id = user.id.unwrap();
        }
        None => {
            println!("User not found");
            return HttpResponse::Unauthorized().json("Invalid credentials");
        }
    }

    /* ====================== TEST DIESEL AREA ======================== */

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
