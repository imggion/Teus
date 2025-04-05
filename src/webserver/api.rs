// teus_webserver/src/main.rs
use crate::webserver::services::systeminfo;
use crate::{config::types::Config, monitor::storage::Storage};
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, http, middleware};
use serde::Serialize;

#[derive(Serialize)]
struct SysInfoResponse {
    timestamp: String,
    cpu_usage: f64,
    ram_usage: f64,
    total_ram: f64,
    free_ram: f64,
    used_swap: f64,
    disks: Vec<DiskInfoResponse>,
}

#[derive(Serialize)]
struct DiskInfoResponse {
    filesystem: String,
    mount_point: String,
    total_space: usize,
    available_space: usize,
    used_space: usize,
}

#[get("/sysinfo")]
async fn sysinfo_handler(config: actix_web::web::Data<Config>) -> impl Responder {
    let storage = match Storage::new(&config.database.path) {
        Ok(storage) => storage,
        Err(e) => {
            eprintln!("Failed to create storage: {}", e);
            return HttpResponse::InternalServerError().json("Failed to connect to database");
        }
    };

    let conn = storage.clone().conn;
    let sys_info = match storage.get_latest_sysinfo(&conn) {
        Ok(sys_info) => sys_info,
        Err(e) => {
            eprintln!("Failed to get latest sysinfo: {}", e);
            return HttpResponse::InternalServerError().json("Failed to get latest sysinfo");
        }
    };

    if let Some(sys_info) = sys_info {
        let timestamp = sys_info.timestamp.clone();
        let disks = sys_info
            .disks
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
            .service(sysinfo_handler)
            .service(systeminfo::get_sysinfo)
    })
    .bind(&url)?
    .run()
    .await
}
