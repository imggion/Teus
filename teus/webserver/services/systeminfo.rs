use crate::webserver::models::sysmodels::{GenericSysInfoResponse, IpInfo, MACInfo};
use actix_web::{get, HttpResponse, Responder};
use sysinfo::{Networks, System};

fn collect_network_info() -> Vec<IpInfo> {
    let networks = Networks::new_with_refreshed_list();
    let mut ip_structs = Vec::new();

    for (interface_name, network) in &networks {
        for ip_network in network.ip_networks() {
            ip_structs.push(IpInfo {
                interface: interface_name.to_string(),
                addr: ip_network.addr.to_string(),
                prefix: ip_network.prefix,
            });
        }
    }

    ip_structs
}

fn collect_mac_address() -> Vec<MACInfo> {
    let networks = Networks::new_with_refreshed_list();
    let mut mac_addresses = Vec::new();
    for (interface_name, network) in &networks {
        mac_addresses.push(MACInfo {
            interface: interface_name.to_string(),
            mac: network.mac_address().to_string(),
        });
    }
    mac_addresses
}

fn convert_seconds_to_date_time(seconds: u64) -> String {
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;

    format!(
        "{} days, {} hours, {} minutes",
        days,
        hours % 24,
        minutes % 60
    )
}

fn get_os_name_version() -> String {
    let os = System::long_os_version();
    match os {
        Some(version) => version,
        None => "No Info".to_string(),
    }
}

/// Returns system information such as hostname, OS details, and network configurations.
///
/// This endpoint provides real-time system information that doesn't need to be stored
/// in the database, as it can be queried directly from the operating system whenever needed.
/// The information returned is transient and reflects the current state of the system
/// rather than persistent data that needs database storage.
#[get("/generic/sysinfo")]
async fn get_sysinfo() -> impl Responder {
    let hostname = System::host_name().unwrap_or_else(|| "No Info".to_string());
    let networks = collect_network_info();
    let mac_addresses = collect_mac_address();
    let os_name = get_os_name_version();

    let uptime_ms = System::uptime();
    let uptime = convert_seconds_to_date_time(uptime_ms);

    let mut response = GenericSysInfoResponse::default();

    response.hostname = hostname;
    response.os = os_name;
    response.uptime = uptime;
    response.kernel_version = System::kernel_version().unwrap_or_else(|| "No Info".to_string());
    response.ipv4 = "No Info".to_string();
    response.networks = networks;
    response.mac_addresses = mac_addresses;

    HttpResponse::Ok().json(response)
}
