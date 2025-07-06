use crate::config::schema;
use actix_web::error::ErrorInternalServerError;
use actix_web::{Error, HttpResponse, get, web};
use teus_database::storage::Storage;
use teus_types::config::{Config, IsFirstVisitResponse};

/* is this really useful? */
#[get("/teus-config")]
pub async fn get_teus_config(config: web::Data<Config>) -> Result<HttpResponse, Error> {
    let storage = Storage::new(&config.database.path).map_err(|e| {
        eprintln!("Failed to initialize storage: {:?}", e); // TODO: Use log::error!
        ErrorInternalServerError("Failed to initialize storage")
    })?;
    let mut conn = storage.diesel_conn.lock().map_err(|_| {
        eprintln!("Mutex poisoned while getting Teus config"); // TODO: Use log::error!
        ErrorInternalServerError("Failed to acquire database lock")
    })?;
    let latest_teusconfig_option =
        schema::TeusConfig::get_teus_server_config(&mut *conn).map_err(|e| {
            eprintln!("Database error getting Teus config: {:?}", e); // TODO: Use log::error!
            ErrorInternalServerError("Error fetching TeusConfig")
        })?;

    match latest_teusconfig_option {
        Some(teus_config) => Ok(HttpResponse::Ok().json(teus_config)),
        None => Ok(HttpResponse::NotFound().json("No TeusConfig found")),
    }
}

#[get("/teus-config/first-visit")]
pub async fn is_first_visit(config: web::Data<Config>) -> Result<HttpResponse, Error> {
    let storage = Storage::new(&config.database.path).map_err(|e| {
        eprintln!(
            "Failed to initialize storage for first-visit check: {:?}",
            e
        ); // TODO: Use log::error!
        ErrorInternalServerError("Failed to initialize storage")
    })?;
    let mut conn = storage.diesel_conn.lock().map_err(|_| {
        eprintln!("Mutex poisoned while checking first visit"); // TODO: Use log::error!
        ErrorInternalServerError("Failed to acquire database lock")
    })?;
    let first_visit = schema::TeusConfig::is_first_visit(&mut *conn).map_err(|e| {
        eprintln!("Database error checking first visit: {:?}", e); // TODO: Use log::error!
        ErrorInternalServerError("Error checking first visit status")
    })?;

    let response = IsFirstVisitResponse { first_visit };
    Ok(HttpResponse::Ok().json(response))
}
