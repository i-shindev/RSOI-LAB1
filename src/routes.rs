use axum::{routing::get, Router};

use crate::handlers;
use crate::repository::PersonRepository;
use crate::service::PersonService;

pub fn router<R: PersonRepository>(service: PersonService<R>) -> Router {
    Router::new()
        .route("/manage/health", get(health))
        .route(
            "/api/v1/persons",
            get(handlers::list_persons::<R>).post(handlers::create_person::<R>),
        )
        .route(
            "/api/v1/persons/{id}",
            get(handlers::get_person::<R>)
                .patch(handlers::update_person::<R>)
                .delete(handlers::delete_person::<R>),
        )
        .with_state(service)
}

async fn health() -> &'static str {
    "OK"
}
