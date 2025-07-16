use crate::errors::ApiError;
use actix_web::{HttpResponse, get, web};
use docker::docker::{DockerClient, DockerError};
use serde::{Deserialize, Serialize};
use serde_qs::to_string;
use std::{fmt::Debug, sync::Mutex};

/* alias to get the life easier */
type DockerState = web::Data<Mutex<Option<DockerClient>>>;

/* get the docker client from the state */
fn get_client(state: &DockerState) -> Result<DockerClient, DockerError> {
    let mut guard = state.lock().unwrap(); // lock the mutex

    if let Some(client) = guard.as_ref() {
        return Ok(client.clone());
    }

    match DockerClient::new(None) {
        Ok(client) => {
            *guard = Some(client.clone());
            Ok(client)
        }
        Err(_) => Err(DockerError::Generic(
            "Could not connect to the Docker socket. Is Docker running?".to_string(),
        )),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericDockerResponse {
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ContainersQuery {
    all: Option<bool>,
}

#[get("/docker/version")]
async fn get_docker_version(docker_state: DockerState) -> Result<HttpResponse, ApiError> {
    let mut docker_client = get_client(&docker_state).map_err(|e| ApiError::Docker(e))?;

    match docker_client.get_version() {
        Ok(version) => Ok(HttpResponse::Ok().json(version)),
        Err(err) => Err(ApiError::Docker(err)),
    }
}

#[get("/docker/containers")]
async fn get_docker_containers(
    query: web::Query<ContainersQuery>,
    docker_state: DockerState,
) -> Result<HttpResponse, ApiError> {
    let query_params: ContainersQuery = query.into_inner();
    let query_string = to_string(&query_params).unwrap();
    let mut docker_client = get_client(&docker_state).map_err(|e| ApiError::Docker(e))?;

    match docker_client.get_containers(Some(query_string)) {
        Ok(containers) => Ok(HttpResponse::Ok().json(containers)),
        Err(err) => Err(ApiError::Docker(err)),
    }
}

#[get("/docker/container/{id}")]
async fn get_docker_container(
    id: web::Path<String>,
    docker_state: DockerState,
) -> Result<HttpResponse, ApiError> {
    let mut docker_client = get_client(&docker_state).map_err(|e| ApiError::Docker(e))?;
    let container_id_clone = id.clone();

    match docker_client.get_container_details(container_id_clone) {
        Ok(container) => Ok(HttpResponse::Ok().json(container)),
        Err(err) => Err(ApiError::Docker(err)),
    }
}

#[get("/docker/volumes")]
async fn get_docker_volumes(docker_state: DockerState) -> Result<HttpResponse, ApiError> {
    let mut docker_client = get_client(&docker_state).map_err(|e| ApiError::Docker(e))?;

    match docker_client.get_volumes() {
        Ok(volumes) => Ok(HttpResponse::Ok().json(volumes)),
        Err(err) => Err(ApiError::Docker(err)),
    }
}

#[get("/docker/volumes/{id}")]
async fn get_docker_volume(
    id: web::Path<String>,
    docker_state: DockerState,
) -> Result<HttpResponse, ApiError> {
    let mut docker_client = get_client(&docker_state).map_err(|e| ApiError::Docker(e))?;

    let cloned_id = id.clone();
    match docker_client.get_volume_details(cloned_id) {
        Ok(volume) => Ok(HttpResponse::Ok().json(volume)),
        Err(err) => Err(ApiError::Docker(err)),
    }
}
