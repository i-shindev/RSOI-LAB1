use super::field_update::FieldUpdate;
use super::input::{NewPersonInput, PersonPatchInput};
use super::person::Person;
use super::validation::{validate_new, validate_patch};

fn ada() -> Person {
    Person {
        id: 1,
        name: "Ada Lovelace".to_owned(),
        age: Some(36),
        address: Some("Marylebone".to_owned()),
        work: Some("Analytical Engine".to_owned()),
    }
}

#[test]
fn a_request_with_only_a_name_is_accepted() {
    let input = NewPersonInput {
        name: Some("Ada Lovelace".to_owned()),
        ..Default::default()
    };

    let person = validate_new(input).expect("a name alone is enough");

    assert_eq!(person.name, "Ada Lovelace");
    assert_eq!(person.age, None);
    assert_eq!(person.address, None);
    assert_eq!(person.work, None);
}

#[test]
fn a_request_without_a_name_is_rejected() {
    let errors = validate_new(NewPersonInput::default()).unwrap_err();

    assert_eq!(
        errors.get("name").map(String::as_str),
        Some("must be present")
    );
}

#[test]
fn every_broken_field_is_reported_at_once() {
    let input = NewPersonInput {
        name: Some("   ".to_owned()),
        age: Some(-1),
        ..Default::default()
    };

    let errors = validate_new(input).unwrap_err();

    assert_eq!(
        errors.get("name").map(String::as_str),
        Some("must not be blank")
    );
    assert_eq!(
        errors.get("age").map(String::as_str),
        Some("must not be negative")
    );
}

#[test]
fn a_patch_touches_only_the_fields_it_carries() {
    let input = PersonPatchInput {
        name: FieldUpdate::Set("Grace Hopper".to_owned()),
        address: FieldUpdate::Clear,
        ..Default::default()
    };
    let mut person = ada();

    validate_patch(input)
        .expect("a valid patch")
        .apply_to(&mut person);

    assert_eq!(person.name, "Grace Hopper");
    assert_eq!(person.address, None);
    assert_eq!(person.age, Some(36));
    assert_eq!(person.work, Some("Analytical Engine".to_owned()));
}

#[test]
fn a_patch_cannot_clear_the_name() {
    let input = PersonPatchInput {
        name: FieldUpdate::Clear,
        ..Default::default()
    };

    let errors = validate_patch(input).unwrap_err();

    assert_eq!(
        errors.get("name").map(String::as_str),
        Some("must not be null")
    );
}
