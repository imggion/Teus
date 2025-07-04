use crate::bookmarks::handlers as bookmark_handlers;
use crate::config::handlers::get_teus_config;
use crate::monitor::query;
use crate::webserver::auth::handlers::{check, login, signup, JwtConfig};
use crate::webserver::auth::middleware::AuthMiddlewareFactory;
use crate::webserver::docker::handlers::{
    get_docker_container, get_docker_containers, get_docker_version, get_docker_volume,
    get_docker_volumes,
};
use crate::webserver::models::sysmodels::{DiskInfoResponse, SysInfoResponse};
use crate::webserver::services::systeminfo;
use crate::{config::types::Config, monitor::storage::Storage};
use actix_cors::Cors;
use actix_web::error::ErrorInternalServerError;
use actix_web::{get, http, middleware, web, App, Error, HttpResponse, HttpServer};

// TODO: move this api into another file `syshandler` or something
#[get("/sysinfo")]
async fn sysinfo_handler(storage: web::Data<Storage>) -> Result<HttpResponse, Error> {
    let mut conn = storage.diesel_conn.lock().map_err(|_| {
        eprintln!("Mutex poisoned while getting sysinfo"); // TODO: Use log::error!
        ErrorInternalServerError("Failed to acquire database lock")
    })?;

    let sys_info_result = query::get_latest_sysinfo_with_disks(&mut conn).map_err(|e| {
        eprintln!("Database error getting sysinfo: {:?}", e); // TODO: Use log::error!
        ErrorInternalServerError("Failed to get latest sysinfo")
    })?;

    if let Some((sys_info, disks)) = sys_info_result {
        let timestamp = sys_info.timestamp.clone();
        let disks = disks
            .iter()
            .map(|d| DiskInfoResponse {
                filesystem: d.filesystem.clone(),
                mount_point: d.mounted_path.clone(),
                total_space: d.size,
                available_space: d.available,
                used_space: d.used,
            })
            .collect();

        let response = SysInfoResponse {
            timestamp,
            cpu_usage: sys_info.cpu_usage,
            ram_usage: sys_info.ram_usage,
            total_ram: sys_info.total_ram,
            free_ram: sys_info.free_ram,
            used_swap: sys_info.used_swap,
            disks: disks,
        };

        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::NotFound().json("No sysinfo found"))
    }
}

#[actix_web::main]
pub async fn start_webserver(config: &Config, storage: Storage) -> std::io::Result<()> {
    let url = format!("{}:{}", config.server.host, config.server.port);
    println!("Webserver listening on {}", url);

    let app_config_data = web::Data::new(config.clone());
    let app_storage_data = web::Data::new(storage.clone()); // Clone for app_data

    // TODO: Put the secret here from the config
    let jwt_secret = config.server.secret.clone();
    let jwt_config = web::Data::new(JwtConfig {
        secret: jwt_secret.to_string(),
        expiration_hours: 24,
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(app_config_data.clone()) // Share config
            .app_data(app_storage_data.clone()) // Share storage
            .app_data(jwt_config.clone()) // Share JWT config
            // Public routes
            .service(
                web::scope("/api/v1/auth")
                    .service(login)
                    .service(signup)
                    .service(get_teus_config),
            )
            // Protected routes
            .service(
                web::scope("/api/v1/teus")
                    .wrap(AuthMiddlewareFactory::new(jwt_secret.to_string()))
                    .service(check) // check for auth
                    .service(sysinfo_handler)
                    .service(systeminfo::get_sysinfo)
                    .service(get_docker_version)
                    .service(get_docker_containers)
                    .service(get_docker_container)
                    .service(get_docker_volume)
                    .service(get_docker_volumes)
                    .service(bookmark_handlers::get_user_services)
                    .service(bookmark_handlers::add_service)
                    .service(bookmark_handlers::delete_service_by_id)
                    .service(bookmark_handlers::update_service_by_id),
            )
    })
    .bind(&url)?
    .run()
    .await
}
