use std::collections::BTreeMap;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::domain::ValidationErrors;
use crate::service::ServiceError;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationErrorResponse {
    pub message: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub errors: ValidationErrors,
}

#[derive(Debug)]
pub enum ApiError {
    Service(ServiceError),
    MalformedBody(String),
}

impl From<ServiceError> for ApiError {
    fn from(error: ServiceError) -> Self {
        ApiError::Service(error)
    }
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        ApiError::MalformedBody(rejection.body_text())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Service(ServiceError::NotFound(id)) => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: format!("Person with id {id} not found"),
                }),
            )
                .into_response(),

            ApiError::Service(ServiceError::Validation(errors)) => (
                StatusCode::BAD_REQUEST,
                Json(ValidationErrorResponse {
                    message: "Validation failed".to_owned(),
                    errors,
                }),
            )
                .into_response(),

            ApiError::Service(ServiceError::Storage(cause)) => {
                tracing::error!("storage failure: {cause}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        message: "Internal server error".to_owned(),
                    }),
                )
                    .into_response()
            }

            ApiError::MalformedBody(message) => (
                StatusCode::BAD_REQUEST,
                Json(ValidationErrorResponse {
                    message,
                    errors: ValidationErrors::new(),
                }),
            )
                .into_response(),
        }
    }
}
