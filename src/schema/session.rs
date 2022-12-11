use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::response::IntoResponse;
use axum::RequestPartsExt;
use axum::{http::request::Parts, response::Response};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, FromRow};
use uuid::Uuid;

use crate::AppState;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub creation: DateTime<Utc>,
}

impl Session {
    async fn fetch(db: &PgPool, id: Uuid) -> Result<Session> {
        let session = sqlx::query_as!(
            Session,
            "SELECT id, creator_id, creation
            FROM session
            WHERE id = $1
            ",
            id
        )
        .fetch_one(db)
        .await?;
        Ok(session)
    }
    async fn list(db: &PgPool, creator_id: Uuid) -> Result<Vec<Session>> {
        let sessions = sqlx::query_as!(
            Session,
            "SELECT id, creator_id, creation
            FROM session
            WHERE creator_id = $1",
            creator_id
        )
        .fetch_all(db)
        .await?;
        Ok(sessions)
    }
    async fn insert(&self, db: &PgPool) -> Result<()> {
        let _ = sqlx::query!(
            "INSERT INTO session
            (id, creator_id, creation)
            VALUES ($1, $2, $3)
            ",
            self.id,
            self.creator_id,
            self.creation,
        )
        .execute(db)
        .await?;
        Ok(())
    }
    async fn delete(&self, db: &PgPool) -> Result<()> {
        let _ = sqlx::query!(
            "DELETE FROM session
            WHERE creator_id = $1 AND ($2::uuid IS NULL OR id = $2)
            ",
            self.creator_id,
            self.id,
        )
        .execute(db)
        .await?;
        Ok(())
    }

    // Funções públicas
    // Essas modelam as lógicas de negócio (por exemplo: precisa estar autenticado para revogar
    // outras autenticações)

    pub async fn create(db: &PgPool, creator_id: Uuid) -> Result<Session> {
        let session = Session {
            id: Uuid::new_v4(),
            creation: Utc::now(),
            creator_id,
        };
        session.insert(db).await?;
        Ok(session)
    }
    pub async fn authenticate(db: &PgPool, session_id: Uuid) -> Result<Session> {
        Session::fetch(db, session_id)
            .await
            .context("Sua sessão expirou, faça login novamente.")
    }
}

/*
#[async_trait]
impl FromRequestParts<AppState> for Session {
    type Rejection = Response;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let State(app_state) = State::<AppState>::from_request_parts(parts, state)
            .await
            .map_err(|e| e.into_response())?;
    }
}
*/
