use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::{
    cookie::{self, Cookie, Key},
    SignedCookieJar,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, FromRow};
use uuid::Uuid;

use crate::{state::AppState, AppError};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub creator: String,
    pub creation: DateTime<Utc>,
}

impl Session {
    pub async fn fetch(db: &PgPool, id: Uuid) -> Result<Self, AppError> {
        sqlx::query_as!(
            Self,
            "SELECT id, creator, creation
            FROM session
            WHERE id = $1
            ",
            id
        )
        .fetch_one(db)
        .await
        .map_err(Into::into)
    }
    pub async fn list(db: &PgPool, creator: &str) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Self,
            "SELECT id, creator, creation
            FROM session
            WHERE creator = $1",
            creator
        )
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }
    pub async fn delete(
        db: &PgPool,
        creator: &str,
        session_id: Option<Uuid>,
    ) -> Result<(), AppError> {
        sqlx::query!(
            "DELETE FROM session
            WHERE creator = $1 AND ($2::uuid IS NULL OR id = $2)
            ",
            creator,
            session_id,
        )
        .execute(db)
        .await
        .map_err(Into::into)
        .map(|_| ())
    }
    pub async fn insert(&self, db: &PgPool) -> Result<(), AppError> {
        sqlx::query!(
            "INSERT INTO session
            (id, creator, creation)
            VALUES ($1, $2, $3)
            ",
            self.id,
            self.creator,
            self.creation,
        )
        .execute(db)
        .await
        .map_err(Into::into)
        .map(|_| ())
    }

    // Funções públicas
    // Essas modelam as lógicas de negócio (por exemplo: precisa estar autenticado para revogar
    // outras autenticações)

    pub async fn create(db: &PgPool, creator: &str) -> Result<Self, AppError> {
        let session = Self {
            id: Uuid::new_v4(),
            creation: Utc::now(),
            creator: creator.into(),
        };
        session.insert(db).await?;
        Ok(session)
    }
    pub async fn authenticate(db: &PgPool, session_id: Uuid) -> Result<Self, AppError> {
        Self::fetch(db, session_id)
            .await
            .map_err(|_| AppError::InvalidSession)
    }
    pub async fn show_all(&self, db: &PgPool) -> Result<Vec<Self>, AppError> {
        Self::list(db, &self.creator).await
    }
    pub async fn revoke(&self, db: &PgPool, session_id: Option<Uuid>) -> Result<(), AppError> {
        Self::delete(db, &self.creator, session_id).await
    }
    pub async fn revoke_self(&self, db: &PgPool) -> Result<(), AppError> {
        self.revoke(db, Some(self.id)).await
    }
    pub async fn revoke_all(&self, db: &PgPool) -> Result<(), AppError> {
        self.revoke(db, None).await
    }
    pub fn as_cookie(&self) -> Cookie<'static> {
        let session_id = self.id.as_simple().to_string();
        Cookie::build("session", session_id)
            // Desabilitar "Scure" nos cookies quando estiver em debug
            // Se não n funciona sem https
            .secure(cfg!(not(debug_assertions)))
            .http_only(true)
            .same_site(cookie::SameSite::Strict)
            .permanent()
            .finish()
            .into_owned()
    }
}

#[async_trait]
impl FromRequestParts<AppState> for Session {
    type Rejection = AppError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let cookie_jar = SignedCookieJar::<Key>::from_request_parts(parts, state)
            .await
            .unwrap(); // It's Infallible

        let session = cookie_jar.get("session").ok_or(AppError::NotAuthenticated)?;
        let uuid = Uuid::parse_str(session.value()).or(Err(AppError::NotAuthenticated))?;
        Session::authenticate(&state.db_pool, uuid).await
    }
}

/*
impl IntoResponseParts for Session {
    type Error = std::convert::Infallible;
    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        let cookies = SignedCookieJar::<Key>::new(res.extensions());
        let cookies = cookies.add(self.as_cookie());
        cookies.into_response_parts(res)
    }
}
*/
