use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use sqlx::PgPool;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db_pool: PgPool,
    pub cookie_key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_key.clone()
    }
}
