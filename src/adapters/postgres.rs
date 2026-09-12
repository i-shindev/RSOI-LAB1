use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, PgPool, Row};

use crate::domain::{NewPerson, Person, PersonPatch, PersonRepository, RepositoryError};

const COLUMNS: &str = "id, name, age, address, work";

impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        RepositoryError(error.to_string())
    }
}

impl<'r> FromRow<'r, PgRow> for Person {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            age: row.try_get("age")?,
            address: row.try_get("address")?,
            work: row.try_get("work")?,
        })
    }
}

pub struct PgPersonRepository {
    pool: PgPool,
}

impl PgPersonRepository {
    pub async fn connect(database_url: &str, max_connections: u32) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }
}

impl PersonRepository for PgPersonRepository {
    async fn list(&self) -> Result<Vec<Person>, RepositoryError> {
        let persons =
            sqlx::query_as::<_, Person>(&format!("SELECT {COLUMNS} FROM persons ORDER BY id"))
                .fetch_all(&self.pool)
                .await?;

        Ok(persons)
    }

    async fn find(&self, id: i32) -> Result<Option<Person>, RepositoryError> {
        let person =
            sqlx::query_as::<_, Person>(&format!("SELECT {COLUMNS} FROM persons WHERE id = $1"))
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(person)
    }

    async fn create(&self, person: NewPerson) -> Result<Person, RepositoryError> {
        let stored = sqlx::query_as::<_, Person>(&format!(
            "INSERT INTO persons (name, age, address, work) \
             VALUES ($1, $2, $3, $4) RETURNING {COLUMNS}"
        ))
        .bind(person.name)
        .bind(person.age)
        .bind(person.address)
        .bind(person.work)
        .fetch_one(&self.pool)
        .await?;

        Ok(stored)
    }

    async fn update(&self, id: i32, patch: PersonPatch) -> Result<Option<Person>, RepositoryError> {
        let mut transaction = self.pool.begin().await?;

        let existing = sqlx::query_as::<_, Person>(&format!(
            "SELECT {COLUMNS} FROM persons WHERE id = $1 FOR UPDATE"
        ))
        .bind(id)
        .fetch_optional(&mut *transaction)
        .await?;

        let Some(mut person) = existing else {
            return Ok(None);
        };
        patch.apply_to(&mut person);

        sqlx::query("UPDATE persons SET name = $2, age = $3, address = $4, work = $5 WHERE id = $1")
            .bind(person.id)
            .bind(&person.name)
            .bind(person.age)
            .bind(&person.address)
            .bind(&person.work)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(Some(person))
    }

    async fn delete(&self, id: i32) -> Result<bool, RepositoryError> {
        let result = sqlx::query("DELETE FROM persons WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
