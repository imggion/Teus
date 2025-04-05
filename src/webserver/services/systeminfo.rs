use crate::config::types::Config;
use actix_web::{HttpResponse, Responder, get};
use sysinfo::{IpNetwork, Networks, System};

#[derive(serde::Serialize, Debug)]
struct GenericSysInfoResponse {
    hostname: String,
    os: String,
    uptime: u64,
    kernel_version: String,
    ipv4: String,
}

impl GenericSysInfoResponse {
    pub fn new(
        hostname: String,
        os: String,
        uptime: u64,
        kernel_version: String,
        ipv4: String,
    ) -> Self {
        Self {
            hostname,
            os,
            uptime,
            kernel_version,
            ipv4,
        }
    }

    pub fn default() -> Self {
        Self {
            hostname: "No Info".to_string(),
            os: "No Info".to_string(),
            uptime: 0,
            kernel_version: "No Info".to_string(),
            ipv4: "No Info".to_string(),
        }
    }
}

#[get("/generic/sysinfo")]
async fn get_sysinfo(config: actix_web::web::Data<Config>) -> impl Responder {
    let hostname = System::host_name().unwrap_or_else(|| "No Info".to_string());
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, network) in &networks {
        println!(
            "Interface: {}, Ip Networks: {:?}",
            interface_name,
            network.ip_networks()
        );
    }

    let mut response = GenericSysInfoResponse::default();
    response.hostname = hostname;
    response.os = System::os_version().unwrap_or_else(|| "No Info".to_string());
    response.uptime = System::uptime();
    response.kernel_version = System::kernel_version().unwrap_or_else(|| "No Info".to_string());
    response.ipv4 = "No Info".to_string();

    HttpResponse::Ok().json(response)
}
