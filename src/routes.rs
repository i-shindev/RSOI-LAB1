use std::sync::Arc;

use axum::{routing::get, Router};

use crate::handlers;
use crate::repository::PersonRepository;

pub fn router<R: PersonRepository>(repository: Arc<R>) -> Router {
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
        .with_state(repository)
}

async fn health() -> &'static str {
    "OK"
}
