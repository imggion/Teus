use crate::{
    config::{
        schema,
        types::{Config, IsFirstVisitResponse},
    },
    monitor::storage::Storage,
};
use actix_web::{HttpResponse, Responder, get};

#[get("/teus-config")]
pub async fn get_teus_config(config: actix_web::web::Data<Config>) -> impl Responder {
    let storage = Storage::new(&config.database.path).unwrap();
    let mut conn = storage.diesel_conn.lock().unwrap();
    let latest_teusconfig_option = schema::TeusConfig::get_teus_server_config(&mut *conn).unwrap();

    match latest_teusconfig_option {
        Some(teus_config) => HttpResponse::Ok().json(teus_config),
        None => HttpResponse::NotFound().json("No TeusConfig found"),
    }
}

#[get("/teus-config/first-visit")]
pub async fn is_first_visit(config: actix_web::web::Data<Config>) -> impl Responder {
    let storage = Storage::new(&config.database.path).unwrap();
    let mut conn = storage.diesel_conn.lock().unwrap();
    let first_visit = schema::TeusConfig::is_first_visit(&mut *conn).unwrap();

    let response = IsFirstVisitResponse { first_visit };

    HttpResponse::Ok().json(response)
}
