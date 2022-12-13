use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, FromRow};
use uuid::Uuid;

use crate::{schema::Session, AppError};

#[derive(Serialize, Deserialize, FromRow, Default)]
pub struct Person {
    pub email: String,
    pub real_name: String,
    pub pix_key: Option<String>,
    #[serde(skip_serializing)]
    password: String,
}

impl Person {
    pub async fn fetch(db: &PgPool, email: &str) -> Result<Self, AppError> {
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
    pub async fn list(db: &PgPool, ride_id: Option<Uuid>) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Self,
            "SELECT email, real_name, pix_key, password
            FROM person
            INNER JOIN rider ON rider.person = person.email
            WHERE ($1::uuid IS NULL OR rider.ride = $1)
            ",
            ride_id
        )
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }
    pub async fn insert(&self, db: &PgPool) -> Result<(), AppError> {
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
    pub async fn _update(&self, db: &PgPool) -> Result<(), AppError> {
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

    pub async fn list_riders(db: &PgPool, ride_id: Uuid) -> Result<Vec<Self>, AppError> {
        Self::list(db, Some(ride_id)).await
    }
    pub async fn get(db: &PgPool, email: &str) -> Result<Self, AppError> {
        Self::fetch(db, email).await
    }

    pub async fn register(
        db: &PgPool,
        email: &str,
        password: &str,
        real_name: &str,
    ) -> Result<Self, AppError> {
        let password = hash_password(&password)?;
        let person = Self {
            email: email.into(),
            password,
            real_name: real_name.into(),
            ..Default::default()
        };
        person.insert(db).await?;
        Ok(person)
    }
    pub async fn login(db: &PgPool, email: &str, password: &str) -> Result<Session, AppError> {
        let person = Self::get(db, email)
            .await
            .or(Err(AppError::InvalidCredentials))?;
        if verify_password(&password, &person.password)? {
            Ok(Session::create(db, email).await?)
        } else {
            Err(AppError::InvalidCredentials)
        }
    }
    pub fn get_pix_qr(&self, amount: Option<Decimal>) -> Option<String> {
        Some(generate_emv_pix(
            self.pix_key.as_ref()?,
            &self.real_name,
            "São Carlos",
            amount,
        ))
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

fn generate_emv_pix(key: &str, name: &str, city: &str, amount: Option<Decimal>) -> String {
    fn emv_field(id: u8, payload: &str) -> String {
        format!("{:02}{:02}{}", id, payload.chars().count(), payload)
    }

    fn calculate_crc(content: &str) -> String {
        let content = format!("{}6304", content);
        const CRC_ALG: crc::Algorithm<u16> = crc::Algorithm {
            width: 16,
            poly: 0x1021,
            init: 0xffff,
            refin: false,
            refout: false,
            xorout: 0,
            check: 0,
            residue: 0,
        };
        let crc = crc::Crc::<u16>::new(&CRC_ALG);
        let mut digest = crc.digest();
        digest.update(content.as_bytes());
        format!("{:X}", digest.finalize())
    }

    let emv_version = emv_field(0, "01");
    let merchant_info = {
        let gui = emv_field(0, "br.gov.bcb.pix");
        let pix = emv_field(1, key);
        emv_field(26, &format!("{gui}{pix}"))
    };
    let category = emv_field(52, "0000");
    let currency = emv_field(53, "986");
    let receiver_amount = amount
        .map(|x| emv_field(54, &x.to_string()))
        .unwrap_or_default();
    let receiver_country = emv_field(58, "BR");
    let receiver_name = emv_field(59, name);
    let receiver_city = emv_field(60, city);
    let additional = emv_field(62, &emv_field(5, "***"));
    let full_code = format!("{emv_version}{merchant_info}{category}{currency}{receiver_amount}{receiver_country}{receiver_name}{receiver_city}{additional}");
    let crc = emv_field(63, &calculate_crc(&full_code));
    let final_code = format!("{full_code}{crc}");

    final_code
}
