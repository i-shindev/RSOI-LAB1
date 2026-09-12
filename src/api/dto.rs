use serde::{Deserialize, Deserializer, Serialize};

use crate::domain::{FieldUpdate, NewPersonInput, Person, PersonPatchInput};

#[derive(Debug, Serialize)]
pub struct PersonResponse {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work: Option<String>,
}

impl From<Person> for PersonResponse {
    fn from(person: Person) -> Self {
        Self {
            id: person.id,
            name: person.name,
            age: person.age,
            address: person.address,
            work: person.work,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct PersonRequest {
    pub name: Option<String>,
    pub age: Option<i32>,
    pub address: Option<String>,
    pub work: Option<String>,
}

impl From<PersonRequest> for NewPersonInput {
    fn from(request: PersonRequest) -> Self {
        Self {
            name: request.name,
            age: request.age,
            address: request.address,
            work: request.work,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct PersonPatchRequest {
    #[serde(default, deserialize_with = "present")]
    pub name: Option<Option<String>>,
    #[serde(default, deserialize_with = "present")]
    pub age: Option<Option<i32>>,
    #[serde(default, deserialize_with = "present")]
    pub address: Option<Option<String>>,
    #[serde(default, deserialize_with = "present")]
    pub work: Option<Option<String>>,
}

impl From<PersonPatchRequest> for PersonPatchInput {
    fn from(request: PersonPatchRequest) -> Self {
        Self {
            name: field_update(request.name),
            age: field_update(request.age),
            address: field_update(request.address),
            work: field_update(request.work),
        }
    }
}

fn field_update<T>(value: Option<Option<T>>) -> FieldUpdate<T> {
    match value {
        None => FieldUpdate::Keep,
        Some(None) => FieldUpdate::Clear,
        Some(Some(value)) => FieldUpdate::Set(value),
    }
}

fn present<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::deserialize(deserializer).map(Some)
}
