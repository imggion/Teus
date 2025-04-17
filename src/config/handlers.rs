use super::query;
use crate::{config::types::Config, monitor::storage::Storage};
use actix_web::{HttpResponse, Responder, get};

#[get("/teus-config")]
pub async fn get_teus_config(config: actix_web::web::Data<Config>) -> impl Responder {
    let storage = Storage::new(&config.database.path).unwrap();
    let mut conn = storage.diesel_conn.lock().unwrap();
    let latest_teusconfig_option = query::get_teus_server_config(&mut *conn).unwrap();

    match latest_teusconfig_option {
        Some(teus_config) => HttpResponse::Ok().json(teus_config),
        None => HttpResponse::NotFound().json("No TeusConfig found"),
    }
}
