use std::future::Future;

use crate::models::{NewPerson, Person, PersonPatch};

#[derive(Debug)]
pub struct RepositoryError(pub String);

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for RepositoryError {}

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

#[cfg(test)]
pub use in_memory::InMemoryPersonRepository;

#[cfg(test)]
mod in_memory {
    use std::collections::BTreeMap;
    use std::sync::{Mutex, MutexGuard};

    use super::{NewPerson, Person, PersonPatch, PersonRepository, RepositoryError};

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
        async fn list(&self) -> Result<Vec<Person>, RepositoryError> {
            Ok(self.state().persons.values().cloned().collect())
        }

        async fn find(&self, id: i32) -> Result<Option<Person>, RepositoryError> {
            Ok(self.state().persons.get(&id).cloned())
        }

        async fn create(&self, person: NewPerson) -> Result<Person, RepositoryError> {
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

        async fn update(
            &self,
            id: i32,
            patch: PersonPatch,
        ) -> Result<Option<Person>, RepositoryError> {
            let mut state = self.state();

            match state.persons.get_mut(&id) {
                Some(person) => {
                    patch.apply_to(person);
                    Ok(Some(person.clone()))
                }
                None => Ok(None),
            }
        }

        async fn delete(&self, id: i32) -> Result<bool, RepositoryError> {
            Ok(self.state().persons.remove(&id).is_some())
        }
    }
}
