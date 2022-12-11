use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, FromRow};
use uuid::Uuid;

use crate::AppError;

#[derive(Serialize, Deserialize, FromRow, Default)]
pub struct Rider {
    ride: Uuid,
    person: String,
    review: Option<bool>,
}

impl Rider {
    async fn fetch(db: &PgPool, ride: Uuid, person: &str) -> Result<Self, AppError> {
        sqlx::query_as!(
            Self,
            "SELECT ride, person, review
            FROM rider
            WHERE ride = $1 AND person = $2
            ",
            ride,
            person
        )
        .fetch_one(db)
        .await
        .map_err(Into::into)
    }
    async fn list(db: &PgPool, person: &str) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Self,
            "SELECT ride, person, review
            FROM rider
            WHERE person = $1",
            person
        )
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }
    async fn delete(db: &PgPool, ride: Uuid, person: &str) -> Result<(), AppError> {
        sqlx::query!(
            "DELETE FROM rider
            WHERE ride = $1 AND person = $2
            ",
            ride,
            person,
        )
        .execute(db)
        .await
        .map_err(Into::into)
        .map(|_| ())
    }
    async fn insert(&self, db: &PgPool) -> Result<(), AppError> {
        sqlx::query!(
            "INSERT INTO rider
            (ride, person, review)
            VALUES ($1, $2, $3)
            ",
            self.ride,
            self.person,
            self.review,
        )
        .execute(db)
        .await
        .map_err(Into::into)
        .map(|_| ())
    }
}
