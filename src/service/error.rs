use crate::domain::ValidationErrors;
use crate::repository::RepositoryError;

#[derive(Debug)]
pub enum ServiceError {
    NotFound(i32),
    Validation(ValidationErrors),
    Storage(String),
}

impl From<ValidationErrors> for ServiceError {
    fn from(errors: ValidationErrors) -> Self {
        ServiceError::Validation(errors)
    }
}

impl From<RepositoryError> for ServiceError {
    fn from(error: RepositoryError) -> Self {
        ServiceError::Storage(error.to_string())
    }
}
