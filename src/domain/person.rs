use super::field_update::FieldUpdate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub id: i32,
    pub name: String,
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PersonPatch {
    pub name: Option<String>,
    pub age: FieldUpdate<i32>,
    pub address: FieldUpdate<String>,
    pub work: FieldUpdate<String>,
}

impl PersonPatch {
    pub fn apply_to(self, person: &mut Person) {
        if let Some(name) = self.name {
            person.name = name;
        }
        self.age.apply(&mut person.age);
        self.address.apply(&mut person.address);
        self.work.apply(&mut person.work);
    }
}
