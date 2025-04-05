use crate::config::types::Config;
use actix_web::{HttpResponse, Responder, get};
use sysinfo::{IpNetwork, Networks, System};

#[derive(serde::Serialize, Debug)]
struct IpInfo {
    pub interface: String,
    pub addr: String,
    pub prefix: u8,
}

#[derive(serde::Serialize, Debug)]
struct MACInfo {
    pub interface: String,
    pub mac: String,
}

#[derive(serde::Serialize, Debug)]
struct GenericSysInfoResponse {
    hostname: String,
    os: String,
    uptime: String,
    kernel_version: String,
    ipv4: String,
    networks: Vec<IpInfo>,
    mac_addresses: Vec<MACInfo>,
}

impl GenericSysInfoResponse {}
impl Default for GenericSysInfoResponse {
    fn default() -> Self {
        Self {
            hostname: "No Info".to_string(),
            os: "No Info".to_string(),
            uptime: "No Info".to_string(),
            kernel_version: "No Info".to_string(),
            ipv4: "No Info".to_string(),
            networks: vec![],
            mac_addresses: vec![],
        }
    }
}

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

/* @Info: We have to make some changes */
/** This section will be updated to include additional system information.
 *  Such as:
 *  - IPV4
 *  - MAC Address
 *  - Network Interface Name
 *  - Network Interface MTU
*/

#[get("/generic/sysinfo")]
async fn get_sysinfo(config: actix_web::web::Data<Config>) -> impl Responder {
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
