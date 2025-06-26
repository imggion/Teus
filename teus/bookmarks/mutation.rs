use super::schema::{NewService, Service};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{RunQueryDsl, SqliteConnection};

impl Service {
    /// Add a new service to the database
    pub fn add_service(
        conn: &mut SqliteConnection,
        new_service: NewService,
    ) -> Result<Service, Error> {
        use crate::schema::services;

        diesel::insert_into(services::table)
            .values(&new_service)
            .returning(Service::as_returning())
            .get_result(conn)
    }

    /// Get all services for a specific user
    pub fn get_services_by_user_id(
        conn: &mut SqliteConnection,
        user_id_claim: i32,
    ) -> Result<Vec<Service>, Error> {
        use crate::schema::services::dsl::*;

        services
            .filter(user_id.eq(user_id_claim))
            .select(Service::as_select())
            .load(conn)
    }

    /// Get a specific service by ID
    pub fn _get_service_by_id(
        conn: &mut SqliteConnection,
        service_id: i32,
    ) -> Result<Service, Error> {
        use crate::schema::services::dsl::*;

        services
            .filter(id.eq(service_id))
            .select(Service::as_select())
            .first(conn)
    }

    /// Update a service
    pub fn update_service(
        conn: &mut SqliteConnection,
        service_id: i32,
        updated_service: NewService,
    ) -> Result<Service, Error> {
        use crate::schema::services::dsl::*;

        diesel::update(services.filter(id.eq(service_id)))
            .set((
                name.eq(&updated_service.name),
                link.eq(&updated_service.link),
                icon.eq(&updated_service.icon),
                user_id.eq(&updated_service.user_id),
            ))
            .returning(Service::as_returning())
            .get_result(conn)
    }

    /// Delete a service
    pub fn delete_service(conn: &mut SqliteConnection, service_id: i32) -> Result<usize, Error> {
        use crate::schema::services::dsl::*;

        diesel::delete(services.filter(id.eq(service_id))).execute(conn)
    }
}
