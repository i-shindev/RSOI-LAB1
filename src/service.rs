use std::collections::BTreeMap;
use std::sync::Arc;

use crate::models::{NewPerson, Person, PersonPatch, PersonPatchRequest, PersonRequest};
use crate::repository::{PersonRepository, RepositoryError};

pub type ValidationErrors = BTreeMap<String, String>;

#[derive(Debug)]
pub enum ServiceError {
    NotFound(i32),
    Validation(ValidationErrors),
    Storage(String),
}

impl From<RepositoryError> for ServiceError {
    fn from(error: RepositoryError) -> Self {
        ServiceError::Storage(error.to_string())
    }
}

pub struct PersonService<R> {
    repository: Arc<R>,
}

impl<R> Clone for PersonService<R> {
    fn clone(&self) -> Self {
        Self {
            repository: Arc::clone(&self.repository),
        }
    }
}

impl<R: PersonRepository> PersonService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn list(&self) -> Result<Vec<Person>, ServiceError> {
        Ok(self.repository.list().await?)
    }

    pub async fn get(&self, id: i32) -> Result<Person, ServiceError> {
        self.repository
            .find(id)
            .await?
            .ok_or(ServiceError::NotFound(id))
    }

    pub async fn create(&self, request: PersonRequest) -> Result<Person, ServiceError> {
        let person = validate_new(request)?;

        Ok(self.repository.create(person).await?)
    }

    pub async fn update(
        &self,
        id: i32,
        request: PersonPatchRequest,
    ) -> Result<Person, ServiceError> {
        let patch = validate_patch(request)?;

        self.repository
            .update(id, patch)
            .await?
            .ok_or(ServiceError::NotFound(id))
    }

    pub async fn delete(&self, id: i32) -> Result<(), ServiceError> {
        self.repository.delete(id).await?;

        Ok(())
    }
}

fn validate_new(request: PersonRequest) -> Result<NewPerson, ServiceError> {
    let mut errors = ValidationErrors::new();

    let name = match request.name {
        Some(name) if !name.trim().is_empty() => Some(name),
        Some(_) => {
            errors.insert("name".to_owned(), "must not be blank".to_owned());
            None
        }
        None => {
            errors.insert("name".to_owned(), "must be present".to_owned());
            None
        }
    };
    validate_age(request.age, &mut errors);

    match name {
        Some(name) if errors.is_empty() => Ok(NewPerson {
            name,
            age: request.age,
            address: request.address,
            work: request.work,
        }),
        _ => Err(ServiceError::Validation(errors)),
    }
}

fn validate_patch(request: PersonPatchRequest) -> Result<PersonPatch, ServiceError> {
    let mut errors = ValidationErrors::new();

    let name = match request.name {
        None => None,
        Some(Some(name)) if !name.trim().is_empty() => Some(name),
        Some(Some(_)) => {
            errors.insert("name".to_owned(), "must not be blank".to_owned());
            None
        }
        Some(None) => {
            errors.insert("name".to_owned(), "must not be null".to_owned());
            None
        }
    };
    validate_age(request.age.flatten(), &mut errors);

    if errors.is_empty() {
        Ok(PersonPatch {
            name,
            age: request.age,
            address: request.address,
            work: request.work,
        })
    } else {
        Err(ServiceError::Validation(errors))
    }
}

fn validate_age(age: Option<i32>, errors: &mut ValidationErrors) {
    if matches!(age, Some(age) if age < 0) {
        errors.insert("age".to_owned(), "must not be negative".to_owned());
    }
}
