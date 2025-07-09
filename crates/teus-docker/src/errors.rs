use std::fmt::Display;

use actix_web::ResponseError;
use docker::docker::DockerError;

#[derive(Debug)]
pub enum ApiError {
    Docker(DockerError),
    Serialization(serde_qs::Error),
    ServiceUnavailable(String),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Docker(e) => write!(f, "An error occurred with the Docker API: {:?}", e),
            ApiError::Serialization(e) => write!(f, "Failed to serialize query parameters: {}", e),
            ApiError::ServiceUnavailable(msg) => write!(f, "{}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::Docker(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Serialization(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::ServiceUnavailable(_) => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}
