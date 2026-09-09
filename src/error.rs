use std::collections::BTreeMap;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub type ValidationErrors = BTreeMap<String, String>;

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
pub enum AppError {
    NotFound(i32),
    Validation(ValidationErrors),
    MalformedBody(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound(id) => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: format!("Person with id {id} not found"),
                }),
            )
                .into_response(),

            AppError::Validation(errors) => (
                StatusCode::BAD_REQUEST,
                Json(ValidationErrorResponse {
                    message: "Validation failed".to_owned(),
                    errors,
                }),
            )
                .into_response(),

            AppError::MalformedBody(message) => (
                StatusCode::BAD_REQUEST,
                Json(ValidationErrorResponse {
                    message,
                    errors: ValidationErrors::new(),
                }),
            )
                .into_response(),

            AppError::Internal(cause) => {
                tracing::error!("internal error: {cause}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        message: "Internal server error".to_owned(),
                    }),
                )
                    .into_response()
            }
        }
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        AppError::MalformedBody(rejection.body_text())
    }
}
