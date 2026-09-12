mod error;

use std::sync::Arc;

use crate::domain::{self, NewPersonInput, Person, PersonPatchInput, PersonRepository};

pub use error::ServiceError;

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

    pub async fn create(&self, input: NewPersonInput) -> Result<Person, ServiceError> {
        let person = domain::validate_new(input)?;

        Ok(self.repository.create(person).await?)
    }

    pub async fn update(
        &self,
        id: i32,
        input: PersonPatchInput,
    ) -> Result<Person, ServiceError> {
        let patch = domain::validate_patch(input)?;

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
