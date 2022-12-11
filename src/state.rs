use sqlx::PgPool;
use tower_cookies::Key;

use crate::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db_pool: PgPool,
    pub cookie_key: Key,
}
