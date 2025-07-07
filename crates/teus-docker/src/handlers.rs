use std::fmt::Debug;

use actix_web::{HttpResponse, Responder, get, web};
use docker::docker::DockerClient;
use serde::{Deserialize, Serialize};
use serde_qs::to_string;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericDockerResponse {
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ContainersQuery {
    all: Option<bool>,
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
async fn get_docker_containers(query: web::Query<ContainersQuery>) -> impl Responder {
    println!("Query: {:?}", query);

    let query_params: ContainersQuery = query.into_inner();
    let query_string = to_string(&query_params).unwrap();

    println!("Query: {:?}", query_string);
    let mut docker_client = DockerClient::new(None);

    match docker_client.get_containers(Some(query_string)) {
        Ok(containers) => HttpResponse::Ok().json(containers),
        Err(err) => HttpResponse::InternalServerError().json(GenericDockerResponse {
            message: format!("Error getting docker containers: {:?}", err),
        }),
    }
}

#[get("/docker/container/{id}")]
async fn get_docker_container(id: web::Path<String>) -> impl Responder {
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
async fn get_docker_volume(id: web::Path<String>) -> impl Responder {
    let mut docker_client = DockerClient::new(None);

    let cloned_id = id.clone();
    match docker_client.get_volume_details(cloned_id) {
        Ok(volume) => HttpResponse::Ok().json(volume),
        Err(err) => HttpResponse::InternalServerError().json(GenericDockerResponse {
            message: format!("Error getting docker volume {} details: {:?}", id, err),
        }),
    }
}
