use crate::bookmarks::schema::{NewService, Service, ServicePayload};
use crate::config::types::Config;
use crate::monitor::storage::Storage;
use crate::webserver::auth::middleware::Claims;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use diesel::Insertable;

#[allow(dead_code)]
/// Helper function to extract claims from request
fn extract_claims_from_request(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    req.extensions().get::<Claims>().cloned().ok_or_else(|| {
        HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "No authentication claims found"
        }))
    })
}

#[get("/bookmarks")]
/// Get all services for the authenticated user
pub async fn get_user_services(
    req: HttpRequest,
    config: actix_web::web::Data<Config>,
) -> impl Responder {
    // Clone the claims to own them
    let claims = match req.extensions().get::<Claims>().cloned() {
        Some(claims) => claims,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "No authentication claims found"
            }));
        }
    };

    let user_id = claims.id;
    let storage = Storage::new(&config.database.path).unwrap();
    let mut conn = storage.diesel_conn.lock().unwrap();
    let services =
        Service::get_services_by_user_id(&mut conn, user_id).expect("Error getting services");

    HttpResponse::Ok().json(serde_json::json!(services))
}

#[post("/bookmarks")]
/// Add a new service for the authenticated user
pub async fn add_service(
    req: HttpRequest,
    service_data: web::Json<ServicePayload>,
    config: actix_web::web::Data<Config>,
) -> impl Responder {

    // if !service_data.values() {}
    let claims = extract_claims_from_request(&req).expect("Cannot extract claims from request");
    let new_service = NewService {
        name: service_data.name.clone(),
        link: service_data.link.clone(),
        icon: service_data.icon.clone(),
        user_id: claims.id,
    };

    let storage = Storage::new(&config.database.path).unwrap();
    let mut conn = storage.diesel_conn.lock().unwrap();

    let service_added = match Service::add_service(&mut conn, new_service) {
        Ok(service) => service,
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "message": "Error creating a new Service",
        }))
    };

    HttpResponse::Created().json(service_added)
}
