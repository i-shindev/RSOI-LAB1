use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

use super::handlers;
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

async fn health() -> Json<Value> {
    Json(json!({
        "status": "OK",
        "commit": std::env::var("RENDER_GIT_COMMIT").ok(),
    }))
}
