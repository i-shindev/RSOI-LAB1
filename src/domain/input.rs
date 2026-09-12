use super::field_update::FieldUpdate;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NewPersonInput {
    pub name: Option<String>,
    pub age: Option<i32>,
    pub address: Option<String>,
    pub work: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PersonPatchInput {
    pub name: FieldUpdate<String>,
    pub age: FieldUpdate<i32>,
    pub address: FieldUpdate<String>,
    pub work: FieldUpdate<String>,
}
