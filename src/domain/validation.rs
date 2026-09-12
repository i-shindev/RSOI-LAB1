use super::error::ValidationErrors;
use super::field_update::FieldUpdate;
use super::input::{NewPersonInput, PersonPatchInput};
use super::person::{NewPerson, PersonPatch};

pub fn validate_new(input: NewPersonInput) -> Result<NewPerson, ValidationErrors> {
    let mut errors = ValidationErrors::new();

    let name = match input.name {
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
    validate_age(input.age, &mut errors);

    match name {
        Some(name) if errors.is_empty() => Ok(NewPerson {
            name,
            age: input.age,
            address: input.address,
            work: input.work,
        }),
        _ => Err(errors),
    }
}

pub fn validate_patch(input: PersonPatchInput) -> Result<PersonPatch, ValidationErrors> {
    let mut errors = ValidationErrors::new();

    let name = match input.name {
        FieldUpdate::Keep => None,
        FieldUpdate::Set(name) if !name.trim().is_empty() => Some(name),
        FieldUpdate::Set(_) => {
            errors.insert("name".to_owned(), "must not be blank".to_owned());
            None
        }
        FieldUpdate::Clear => {
            errors.insert("name".to_owned(), "must not be null".to_owned());
            None
        }
    };
    validate_age(input.age.value().copied(), &mut errors);

    if errors.is_empty() {
        Ok(PersonPatch {
            name,
            age: input.age,
            address: input.address,
            work: input.work,
        })
    } else {
        Err(errors)
    }
}

fn validate_age(age: Option<i32>, errors: &mut ValidationErrors) {
    if matches!(age, Some(age) if age < 0) {
        errors.insert("age".to_owned(), "must not be negative".to_owned());
    }
}
