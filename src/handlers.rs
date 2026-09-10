use axum::{
    extract::{rejection::JsonRejection, Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use crate::error::ApiError;
use crate::models::{Person, PersonPatchRequest, PersonRequest};
use crate::repository::PersonRepository;
use crate::service::PersonService;

pub async fn list_persons<R: PersonRepository>(
    State(service): State<PersonService<R>>,
) -> Result<Json<Vec<Person>>, ApiError> {
    Ok(Json(service.list().await?))
}

pub async fn get_person<R: PersonRepository>(
    State(service): State<PersonService<R>>,
    Path(id): Path<i32>,
) -> Result<Json<Person>, ApiError> {
    Ok(Json(service.get(id).await?))
}

pub async fn create_person<R: PersonRepository>(
    State(service): State<PersonService<R>>,
    body: Result<Json<PersonRequest>, JsonRejection>,
) -> Result<Response, ApiError> {
    let Json(request) = body?;
    let person = service.create(request).await?;

    Ok((
        StatusCode::CREATED,
        [(header::LOCATION, format!("/api/v1/persons/{}", person.id))],
    )
        .into_response())
}

pub async fn update_person<R: PersonRepository>(
    State(service): State<PersonService<R>>,
    Path(id): Path<i32>,
    body: Result<Json<PersonPatchRequest>, JsonRejection>,
) -> Result<Json<Person>, ApiError> {
    let Json(request) = body?;

    Ok(Json(service.update(id, request).await?))
}

pub async fn delete_person<R: PersonRepository>(
    State(service): State<PersonService<R>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ApiError> {
    service.delete(id).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::body::Body;
    use axum::http::{HeaderMap, Request};
    use axum::Router;
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    use super::*;
    use crate::repository::InMemoryPersonRepository;
    use crate::routes;

    const ADA: &str =
        r#"{"name":"Ada Lovelace","age":36,"address":"Marylebone","work":"Analytical Engine"}"#;

    fn app() -> Router {
        routes::router(PersonService::new(Arc::new(
            InMemoryPersonRepository::new(),
        )))
    }

    fn with_body(method: &str, uri: &str, body: &str) -> Request<Body> {
        Request::builder()
            .method(method)
            .uri(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_owned()))
            .expect("well formed request")
    }

    fn without_body(method: &str, uri: &str) -> Request<Body> {
        Request::builder()
            .method(method)
            .uri(uri)
            .body(Body::empty())
            .expect("well formed request")
    }

    async fn call(app: &Router, request: Request<Body>) -> (StatusCode, HeaderMap, Value) {
        let response = app
            .clone()
            .oneshot(request)
            .await
            .expect("the router always responds");

        let status = response.status();
        let headers = response.headers().clone();
        let body = response
            .into_body()
            .collect()
            .await
            .expect("the body is readable")
            .to_bytes();

        let json = if body.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&body).expect("the body is JSON")
        };

        (status, headers, json)
    }

    async fn store_ada(app: &Router) -> i32 {
        let (status, headers, _) = call(app, with_body("POST", "/api/v1/persons", ADA)).await;
        assert_eq!(status, StatusCode::CREATED);

        headers
            .get(header::LOCATION)
            .expect("Location header")
            .to_str()
            .expect("printable Location")
            .rsplit('/')
            .next()
            .expect("id at the end of Location")
            .parse()
            .expect("numeric id")
    }

    #[tokio::test]
    async fn creating_a_person_answers_201_with_its_location() {
        let app = app();

        let (status, headers, body) = call(&app, with_body("POST", "/api/v1/persons", ADA)).await;

        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(
            headers.get(header::LOCATION).map(|l| l.to_str().unwrap()),
            Some("/api/v1/persons/1")
        );
        assert_eq!(body, Value::Null);
    }

    #[tokio::test]
    async fn reading_a_person_answers_the_stored_one() {
        let app = app();
        let id = store_ada(&app).await;

        let (status, _, body) =
            call(&app, without_body("GET", &format!("/api/v1/persons/{id}"))).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            body,
            json!({
                "id": id,
                "name": "Ada Lovelace",
                "age": 36,
                "address": "Marylebone",
                "work": "Analytical Engine"
            })
        );
    }

    #[tokio::test]
    async fn listing_persons_answers_every_stored_one() {
        let app = app();
        let first = store_ada(&app).await;
        let second = store_ada(&app).await;

        let (status, _, body) = call(&app, without_body("GET", "/api/v1/persons")).await;

        assert_eq!(status, StatusCode::OK);
        let listed: Vec<i32> = body
            .as_array()
            .expect("an array of persons")
            .iter()
            .map(|person| person["id"].as_i64().expect("numeric id") as i32)
            .collect();
        assert_eq!(listed, vec![first, second]);
    }

    #[tokio::test]
    async fn patching_a_person_leaves_the_untouched_fields_alone() {
        let app = app();
        let id = store_ada(&app).await;

        let (status, _, body) = call(
            &app,
            with_body(
                "PATCH",
                &format!("/api/v1/persons/{id}"),
                r#"{"name":"Grace Hopper","address":"Arlington"}"#,
            ),
        )
        .await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            body,
            json!({
                "id": id,
                "name": "Grace Hopper",
                "age": 36,
                "address": "Arlington",
                "work": "Analytical Engine"
            })
        );
    }

    #[tokio::test]
    async fn deleting_a_person_answers_204_and_it_is_gone() {
        let app = app();
        let id = store_ada(&app).await;

        let (status, _, body) =
            call(&app, without_body("DELETE", &format!("/api/v1/persons/{id}"))).await;

        assert_eq!(status, StatusCode::NO_CONTENT);
        assert_eq!(body, Value::Null);

        let (status, _, _) =
            call(&app, without_body("GET", &format!("/api/v1/persons/{id}"))).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }
}
