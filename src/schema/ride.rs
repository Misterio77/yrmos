use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, FromRow};
use uuid::Uuid;

use crate::{schema::person::Person, AppError};

#[derive(Serialize, Deserialize, FromRow, Default)]
pub struct Ride {
    pub id: Uuid,
    pub driver: String,
    pub seats: i32,
    pub departure: DateTime<Utc>,
    pub start_location: String,
    pub end_location: String,
    pub cost: Option<Decimal>,
}

impl Ride {
    pub async fn _fetch(db: &PgPool, id: Uuid) -> Result<Self, AppError> {
        sqlx::query_as!(
            Self,
            "SELECT id, driver, seats, departure, start_location, end_location, cost
            FROM ride
            WHERE id = $1
            ",
            id
        )
        .fetch_one(db)
        .await
        .map_err(Into::into)
    }
    pub async fn _list(
        db: &PgPool,
        driver: &str,
        future_only: bool,
    ) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Self,
            "SELECT id, driver, seats, departure, start_location, end_location, cost
            FROM ride
            WHERE driver = $1 AND (NOT $2 OR departure > NOW())",
            driver,
            future_only
        )
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }
    pub async fn insert(&self, db: &PgPool) -> Result<(), AppError> {
        sqlx::query!(
            "INSERT INTO ride
            (id, driver, seats, departure, start_location, end_location, cost)
            VALUES ($1, $2, $3, $4, $5, $6, $7)",
            self.id,
            self.driver,
            self.seats,
            self.departure,
            self.start_location,
            self.end_location,
            self.cost,
        )
        .execute(db)
        .await
        .map_err(Into::into)
        .map(|_| ())
    }

    pub async fn create(
        db: &PgPool,
        driver: &Person,
        seats: i32,
        departure: DateTime<Utc>,
        start_location: String,
        end_location: String,
        cost: Option<Decimal>,
    ) -> Result<Self, AppError> {
        let ride = Self {
            id: Uuid::new_v4(),
            driver: String::from(&driver.email),
            seats,
            departure,
            start_location,
            end_location,
            cost,
        };
        ride.insert(db).await?;
        Ok(ride)
    }

    pub async fn get_driver(&self, db: &PgPool) -> Result<Person, AppError> {
        Person::get(db, &self.driver).await
    }
    pub async fn get_riders(&self, db: &PgPool) -> Result<Vec<Person>, AppError> {
        Person::list_riders(db, self.id).await
    }
}
