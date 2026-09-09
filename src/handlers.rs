use std::sync::Arc;

use axum::{
    extract::{rejection::JsonRejection, Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use crate::error::AppError;
use crate::models::{Person, PersonPatchRequest, PersonRequest};
use crate::repository::PersonRepository;

pub async fn list_persons<R: PersonRepository>(
    State(repository): State<Arc<R>>,
) -> Result<Json<Vec<Person>>, AppError> {
    Ok(Json(repository.list().await?))
}

pub async fn get_person<R: PersonRepository>(
    State(repository): State<Arc<R>>,
    Path(id): Path<i32>,
) -> Result<Json<Person>, AppError> {
    repository
        .find(id)
        .await?
        .map(Json)
        .ok_or(AppError::NotFound(id))
}

pub async fn create_person<R: PersonRepository>(
    State(repository): State<Arc<R>>,
    body: Result<Json<PersonRequest>, JsonRejection>,
) -> Result<Response, AppError> {
    let Json(request) = body?;
    let person = repository.create(request.validate()?).await?;

    Ok((
        StatusCode::CREATED,
        [(header::LOCATION, format!("/api/v1/persons/{}", person.id))],
    )
        .into_response())
}

pub async fn update_person<R: PersonRepository>(
    State(repository): State<Arc<R>>,
    Path(id): Path<i32>,
    body: Result<Json<PersonPatchRequest>, JsonRejection>,
) -> Result<Json<Person>, AppError> {
    let Json(request) = body?;

    repository
        .update(id, request.validate()?)
        .await?
        .map(Json)
        .ok_or(AppError::NotFound(id))
}

pub async fn delete_person<R: PersonRepository>(
    State(repository): State<Arc<R>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    repository.delete(id).await?;

    Ok(StatusCode::NO_CONTENT)
}
