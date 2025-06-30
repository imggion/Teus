use crate::bookmarks::schema::{NewService, Service, ServicePatchPayload, ServicePayload};
use crate::config::types::Config;
use crate::monitor::storage::Storage;
use crate::webserver::auth::middleware::Claims;
use actix_web::{delete, get, patch, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};

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
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Error creating a new Service",
            }))
        }
    };

    HttpResponse::Created().json(service_added)
}

#[delete("/bookmarks/{id}")]
pub async fn delete_service_by_id(
    id: web::Path<i32>,
    req: HttpRequest,
    config: actix_web::web::Data<Config>,
) -> impl Responder {
    let claims = extract_claims_from_request(&req).expect("Cannot extract claims from request");
    let user_id = claims.id;
    let bookmark_id = id.clone();

    let storage = Storage::new(&config.database.path).unwrap();
    let mut conn = storage.diesel_conn.lock().unwrap();

    match Service::_get_service_by_id(&mut conn, bookmark_id) {
        Ok(service) => {
            if service.user_id != user_id {
                return HttpResponse::Unauthorized().json(serde_json::json!({
                    "message": "You are not authorized to delete this service"
                }));
            }

            match Service::delete_service(&mut conn, bookmark_id, user_id) {
                Ok(rows_affected) => {
                    if rows_affected > 0 {
                        HttpResponse::NoContent().finish()
                    } else {
                        HttpResponse::InternalServerError().json(serde_json::json!({
                            "message": "Unexpected error during deletion"
                        }))
                    }
                }
                Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "message": "Error deleting bookmark"
                })),
            }
        }
        Err(_) => {
            // Service doesn't exist
            HttpResponse::NotFound().json(serde_json::json!({
                "message": "Service not found"
            }))
        }
    }
}

#[patch("/bookmarks/{id}")]
pub async fn update_service_by_id(
    id: web::Path<i32>,
    service_data: web::Json<ServicePatchPayload>,
    req: HttpRequest,
    config: actix_web::web::Data<Config>,
) -> impl Responder {
    let claims = extract_claims_from_request(&req).expect("Cannot extract claims from request");
    let user_id = claims.id;
    let bookmark_id = id.clone();

    let storage = Storage::new(&config.database.path).unwrap();
    let mut conn = storage.diesel_conn.lock().unwrap();

    match Service::_get_service_by_id(&mut conn, bookmark_id) {
        Ok(service) => {
            if service.user_id != user_id {
                return HttpResponse::Unauthorized().json(serde_json::json!({
                    "message": "You are not authorized to update this service"
                }));
            }

            match Service::patch_service(&mut conn, bookmark_id, user_id, service_data.into_inner())
            {
                Ok(service) => HttpResponse::Ok().json(service),
                Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "message": "Error updating bookmark"
                })),
            }
        }
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Service not found"
        })),
    }
}
