use std::collections::BTreeMap;
use std::future::Future;
use std::sync::{Mutex, MutexGuard};

use crate::error::AppError;
use crate::models::{NewPerson, Person, PersonPatch};

pub trait PersonRepository: Send + Sync + 'static {
    fn list(&self) -> impl Future<Output = Result<Vec<Person>, AppError>> + Send;

    fn find(&self, id: i32) -> impl Future<Output = Result<Option<Person>, AppError>> + Send;

    fn create(&self, person: NewPerson) -> impl Future<Output = Result<Person, AppError>> + Send;

    fn update(
        &self,
        id: i32,
        patch: PersonPatch,
    ) -> impl Future<Output = Result<Option<Person>, AppError>> + Send;

    fn delete(&self, id: i32) -> impl Future<Output = Result<bool, AppError>> + Send;
}

#[derive(Debug, Default)]
struct State {
    persons: BTreeMap<i32, Person>,
    last_id: i32,
}

#[derive(Debug, Default)]
pub struct InMemoryPersonRepository {
    state: Mutex<State>,
}

impl InMemoryPersonRepository {
    pub fn new() -> Self {
        Self::default()
    }

    fn state(&self) -> MutexGuard<'_, State> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl PersonRepository for InMemoryPersonRepository {
    async fn list(&self) -> Result<Vec<Person>, AppError> {
        Ok(self.state().persons.values().cloned().collect())
    }

    async fn find(&self, id: i32) -> Result<Option<Person>, AppError> {
        Ok(self.state().persons.get(&id).cloned())
    }

    async fn create(&self, person: NewPerson) -> Result<Person, AppError> {
        let mut state = self.state();

        state.last_id += 1;
        let stored = Person {
            id: state.last_id,
            name: person.name,
            age: person.age,
            address: person.address,
            work: person.work,
        };
        state.persons.insert(stored.id, stored.clone());

        Ok(stored)
    }

    async fn update(&self, id: i32, patch: PersonPatch) -> Result<Option<Person>, AppError> {
        let mut state = self.state();

        match state.persons.get_mut(&id) {
            Some(person) => {
                patch.apply_to(person);
                Ok(Some(person.clone()))
            }
            None => Ok(None),
        }
    }

    async fn delete(&self, id: i32) -> Result<bool, AppError> {
        Ok(self.state().persons.remove(&id).is_some())
    }
}
