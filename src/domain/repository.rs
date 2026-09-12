use std::future::Future;

use super::error::RepositoryError;
use super::person::{NewPerson, Person, PersonPatch};

pub trait PersonRepository: Send + Sync + 'static {
    fn list(&self) -> impl Future<Output = Result<Vec<Person>, RepositoryError>> + Send;

    fn find(&self, id: i32) -> impl Future<Output = Result<Option<Person>, RepositoryError>> + Send;

    fn create(
        &self,
        person: NewPerson,
    ) -> impl Future<Output = Result<Person, RepositoryError>> + Send;

    fn update(
        &self,
        id: i32,
        patch: PersonPatch,
    ) -> impl Future<Output = Result<Option<Person>, RepositoryError>> + Send;

    fn delete(&self, id: i32) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
