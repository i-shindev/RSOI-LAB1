use serde::{Deserialize, Deserializer, Serialize};

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
    pub age: Option<Option<i32>>,
    pub address: Option<Option<String>>,
    pub work: Option<Option<String>>,
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
