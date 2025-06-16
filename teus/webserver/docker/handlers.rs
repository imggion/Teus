use actix_web::{get, HttpResponse, Responder};
use docker::docker::DockerClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericDockerResponse {
    message: String,
}

#[get("/docker/version")]
async fn get_docker_version() -> impl Responder {
    let mut docker_client = DockerClient::new(None);
    match docker_client.get_version() {
        Ok(version) => HttpResponse::Ok().json(version),
        Err(err) => HttpResponse::InternalServerError().json(GenericDockerResponse {
            message: format!("Error getting docker version: {:?}", err),
        }),
    }
}

#[get("/docker/containers")]
async fn get_docker_containers() -> impl Responder {
    let mut docker_client = DockerClient::new(None);
    match docker_client.get_containers() {
        Ok(containers) => HttpResponse::Ok().json(containers),
        Err(err) => HttpResponse::InternalServerError().json(GenericDockerResponse {
            message: format!("Error getting docker containers: {:?}", err),
        }),
    }
}

#[get("/docker/container/{id}")]
async fn get_docker_container(id: String) -> impl Responder {
    let mut docker_client = DockerClient::new(None);
    let container_id_clone = id.clone();
    match docker_client.get_container_details(container_id_clone) {
        Ok(container) => HttpResponse::Ok().json(container),
        Err(err) => HttpResponse::InternalServerError().json(GenericDockerResponse {
            message: format!("Error getting docker container {}: {:?}", id, err),
        }),
    }
}

#[get("/docker/volumes")]
async fn get_docker_volumes() -> impl Responder {
    let mut docker_client = DockerClient::new(None);
    match docker_client.get_volumes() {
        Ok(volumes) => HttpResponse::Ok().json(volumes),
        Err(err) => HttpResponse::InternalServerError().json(GenericDockerResponse {
            message: format!("Error getting docker volumes: {:?}", err),
        }),
    }
}

// FIXME: The id is not being passed correctly
// I think the problem is in the enum DockerApi
#[get("/docker/volumes/{id}")]
async fn get_docker_volume(id: String) -> impl Responder {
    let mut docker_client = DockerClient::new(None);
    let cloned_id = id.clone();
    match docker_client.get_volume_details(cloned_id) {
        Ok(volume) => HttpResponse::Ok().json(volume),
        Err(err) => HttpResponse::InternalServerError().json(GenericDockerResponse {
            message: format!("Error getting docker volume {} details: {:?}", id, err),
        }),
    }
}
