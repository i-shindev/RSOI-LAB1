use serde::{Deserialize, Deserializer, Serialize};

use crate::error::{AppError, ValidationErrors};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct PersonRequest {
    pub name: Option<String>,
    pub age: Option<i32>,
    pub address: Option<String>,
    pub work: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewPerson {
    pub name: String,
    pub age: Option<i32>,
    pub address: Option<String>,
    pub work: Option<String>,
}

impl PersonRequest {
    pub fn validate(self) -> Result<NewPerson, AppError> {
        let mut errors = ValidationErrors::new();

        let name = match self.name {
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
        validate_age(self.age, &mut errors);

        match name {
            Some(name) if errors.is_empty() => Ok(NewPerson {
                name,
                age: self.age,
                address: self.address,
                work: self.work,
            }),
            _ => Err(AppError::Validation(errors)),
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

fn present<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::deserialize(deserializer).map(Some)
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PersonPatch {
    pub name: Option<String>,
    pub age: Option<Option<i32>>,
    pub address: Option<Option<String>>,
    pub work: Option<Option<String>>,
}

impl PersonPatchRequest {
    pub fn validate(self) -> Result<PersonPatch, AppError> {
        let mut errors = ValidationErrors::new();

        let name = match self.name {
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
        if let Some(age) = self.age.flatten() {
            validate_age(Some(age), &mut errors);
        }

        if errors.is_empty() {
            Ok(PersonPatch {
                name,
                age: self.age,
                address: self.address,
                work: self.work,
            })
        } else {
            Err(AppError::Validation(errors))
        }
    }
}

impl PersonPatch {
    pub fn apply_to(self, person: &mut Person) {
        if let Some(name) = self.name {
            person.name = name;
        }
        if let Some(age) = self.age {
            person.age = age;
        }
        if let Some(address) = self.address {
            person.address = address;
        }
        if let Some(work) = self.work {
            person.work = work;
        }
    }
}

fn validate_age(age: Option<i32>, errors: &mut ValidationErrors) {
    if matches!(age, Some(age) if age < 0) {
        errors.insert("age".to_owned(), "must not be negative".to_owned());
    }
}
