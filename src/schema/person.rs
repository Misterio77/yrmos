use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, FromRow};

use crate::{AppError, schema::Session};

#[derive(Serialize, Deserialize, FromRow, Default)]
pub struct Person {
    pub email: String,
    pub real_name: String,
    pub pix_key: Option<String>,
    #[serde(skip_serializing)]
    password: String,
}

impl Person {
    async fn fetch(db: &PgPool, email: &str) -> Result<Self, AppError> {
        sqlx::query_as!(
            Self,
            "SELECT email, real_name, pix_key, password
            FROM person
            WHERE email = $1
            ",
            email
        )
        .fetch_one(db)
        .await
        .map_err(Into::into)
    }
    async fn insert(&self, db: &PgPool) -> Result<(), AppError> {
        sqlx::query!(
            "INSERT INTO person
            (email, real_name, pix_key, password)
            VALUES ($1, $2, $3, $4)
            ",
            self.email,
            self.real_name,
            self.pix_key,
            self.password
        )
        .execute(db)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }
    async fn _update(&self, db: &PgPool) -> Result<(), AppError> {
        sqlx::query!(
            "UPDATE person SET
            email = $1,
            real_name = $2,
            pix_key = $3,
            password = $4",
            self.email,
            self.real_name,
            self.pix_key,
            self.password
        )
        .execute(db)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }
    pub async fn get(db: &PgPool, email: &str) -> Result<Self, AppError> {
        Self::fetch(db, email).await
    }
    pub async fn register(
        db: &PgPool,
        email: String,
        password: String,
        real_name: String,
    ) -> Result<Self, AppError> {
        let password = hash_password(&password)?;
        let person = Self {
            email,
            password,
            real_name,
            ..Default::default()
        };
        person.insert(db).await?;
        Ok(person)
    }
    pub async fn login(db: &PgPool, email: String, password: String) -> Result<Session, AppError> {
        let person = Self::get(db, &email).await?;
        if verify_password(&password, &person.password)? {
            Ok(Session::create(db, &email).await?)
        } else {
            Err(AppError::NotAuthenticated)
        }
    }
}

fn hash_password(password: &str) -> Result<String, argon2::Error> {
    use argon2::Config;
    use rand::Rng;

    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    argon2::hash_encoded(password.as_bytes(), &salt, &config)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password.as_bytes())
}
