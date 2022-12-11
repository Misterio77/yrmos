use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, FromRow};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Person {
    pub id: Uuid,
    pub real_name: String,
    pub pix_key: Option<String>,
    #[serde(skip_serializing)]
    email: String,
    #[serde(skip_serializing)]
    password: String,
}

impl Person {
    async fn fetch(db: &PgPool, id: Uuid) -> Result<Person> {
        let person = sqlx::query_as!(
            Person,
            "SELECT id, real_name, pix_key, email, password
            FROM person
            WHERE id = $1
            ",
            id
        )
        .fetch_one(db)
        .await?;
        Ok(person)
    }
    async fn insert(&self, db: &PgPool) -> Result<()> {
        let _ = sqlx::query!(
            "INSERT INTO person
            (id, real_name, pix_key, email, password)
            VALUES ($1, $2, $3, $4, $5)
            ",
            self.id,
            self.real_name,
            self.pix_key,
            self.email,
            self.password
        )
        .execute(db)
        .await?;
        Ok(())
    }
    async fn update(&self, db: &PgPool) -> Result<()> {
        let _ = sqlx::query!(
            "UPDATE person SET
            id = $1,
            real_name = $2,
            pix_key = $3,
            email = $4,
            password = $5",
            self.id,
            self.real_name,
            self.pix_key,
            self.email,
            self.password

        )
        .execute(db)
        .await?;
        Ok(())
    }
}
