use axum::{
    extract::{rejection::JsonRejection, Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use super::dto::{PersonPatchRequest, PersonRequest, PersonResponse};
use super::error::ApiError;
use crate::repository::PersonRepository;
use crate::service::PersonService;

pub async fn list_persons<R: PersonRepository>(
    State(service): State<PersonService<R>>,
) -> Result<Json<Vec<PersonResponse>>, ApiError> {
    let persons = service.list().await?;

    Ok(Json(persons.into_iter().map(PersonResponse::from).collect()))
}

pub async fn get_person<R: PersonRepository>(
    State(service): State<PersonService<R>>,
    Path(id): Path<i32>,
) -> Result<Json<PersonResponse>, ApiError> {
    let person = service.get(id).await?;

    Ok(Json(PersonResponse::from(person)))
}

pub async fn create_person<R: PersonRepository>(
    State(service): State<PersonService<R>>,
    body: Result<Json<PersonRequest>, JsonRejection>,
) -> Result<Response, ApiError> {
    let Json(request) = body?;
    let person = service.create(request.into()).await?;

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
) -> Result<Json<PersonResponse>, ApiError> {
    let Json(request) = body?;
    let person = service.update(id, request.into()).await?;

    Ok(Json(PersonResponse::from(person)))
}

pub async fn delete_person<R: PersonRepository>(
    State(service): State<PersonService<R>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ApiError> {
    service.delete(id).await?;

    Ok(StatusCode::NO_CONTENT)
}
