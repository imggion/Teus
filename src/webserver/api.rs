use crate::config::handlers::get_teus_config;
use crate::monitor::query;
use crate::webserver::auth::handlers::{login, signup, JwtConfig};
use crate::webserver::auth::middleware::AuthMiddlewareFactory;
use crate::webserver::models::sysmodels::{DiskInfoResponse, SysInfoResponse};
use crate::webserver::services::systeminfo;
use crate::{config::types::Config, monitor::storage::Storage};
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, http, middleware, web};

#[get("/sysinfo")]
async fn sysinfo_handler(config: actix_web::web::Data<Config>) -> impl Responder {
    let storage = match Storage::new(&config.database.path) {
        Ok(storage) => storage,
        Err(e) => {
            eprintln!("Failed to create storage: {}", e);
            return HttpResponse::InternalServerError().json("Failed to connect to database");
        }
    };

    let mut conn = storage.diesel_conn.lock().unwrap();
    let sys_info = match query::get_latest_sysinfo_with_disks(&mut conn) {
        Ok(sys_info) => sys_info,
        Err(e) => {
            eprintln!("Failed to get latest sysinfo: {}", e);
            return HttpResponse::InternalServerError().json("Failed to get latest sysinfo");
        }
    };

    if let Some((sys_info, disks)) = sys_info {
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

        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::NotFound().json("No sysinfo found")
    }
}

#[actix_web::main]
pub async fn start_webserver(config: &Config) -> std::io::Result<()> {
    let url = format!("{}:{}", config.server.host, config.server.port);
    println!("Webserver listening on {}", url);

    let config_data = config.clone();
    // TODO: Put the secret here from the config
    let jwt_secret = config_data.server.secret.clone();
    let jwt_config = web::Data::new(JwtConfig {
        secret: jwt_secret.to_string(),
        expiration_hours: 24,
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(actix_web::web::Data::new(config_data.clone()))
            .app_data(jwt_config.clone())
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
                    .service(sysinfo_handler)
                    .service(systeminfo::get_sysinfo),
            )
    })
    .bind(&url)?
    .run()
    .await
}
