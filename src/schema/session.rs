use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, FromRow};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub creator: String,
    pub creation: DateTime<Utc>,
}

impl Session {
    async fn fetch(db: &PgPool, id: Uuid) -> Result<Self> {
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
    async fn list(db: &PgPool, creator: &str) -> Result<Vec<Self>> {
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
    async fn delete(db: &PgPool, creator: &str, session_id: Option<Uuid>) -> Result<()> {
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
    async fn insert(&self, db: &PgPool) -> Result<()> {
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

    pub async fn create(db: &PgPool, creator: &str) -> Result<Self> {
        let session = Self {
            id: Uuid::new_v4(),
            creation: Utc::now(),
            creator: creator.into(),
        };
        session.insert(db).await?;
        Ok(session)
    }
    pub async fn authenticate(db: &PgPool, session_id: Uuid) -> Result<Self> {
        Self::fetch(db, session_id).await
    }
    pub async fn show_all(&self, db: &PgPool) -> Result<Vec<Self>> {
        Self::list(db, &self.creator).await
    }
    pub async fn revoke(&self, db: &PgPool, session_id: Option<Uuid>) -> Result<()> {
        Self::delete(db, &self.creator, session_id).await
    }
    pub async fn revoke_self(self, db: &PgPool) -> Result<()> {
        self.revoke(db, Some(self.id)).await
    }
    pub async fn revoke_all(self, db: &PgPool) -> Result<()> {
        self.revoke(db, None).await
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
