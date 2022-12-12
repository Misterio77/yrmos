use clap::Parser;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::{IpAddr, SocketAddr};
use axum_extra::extract::cookie::Key;

use crate::{errors::AppError, state::AppState};

#[derive(Parser, Clone)]
pub struct AppConfig {
    #[clap(long, default_value = "0.0.0.0", env = "YRMOS_ADDRESS")]
    pub address: IpAddr,
    #[clap(long, default_value = "8080", env = "YRMOS_PORT")]
    pub port: u16,
    #[clap(long, default_value = "postgres:///yrmos", env = "YRMOS_DATABASE")]
    pub database_url: String,
    #[clap(long, default_value = "info", env = "YRMOS_DATABASE")]
    pub log_level: LevelFilter,
    #[clap(long, env = "YRMOS_SECRET_KEY")]
    pub secret_key: Option<String>,
}

impl AppConfig {
    pub async fn connect_db(&self) -> Result<PgPool, AppError> {
        Ok(PgPoolOptions::new().connect(&self.database_url).await?)
    }
    pub async fn get_cookie_key(&self) -> Result<Key, AppError> {
        Ok(match &self.secret_key {
            Some(key) => Key::from(key.as_bytes()),
            None => Key::generate(),
        })
    }
    pub fn get_logger(&self) -> SimpleLogger {
        SimpleLogger::new().with_level(self.log_level)
    }
    pub fn get_addr(&self) -> SocketAddr {
        SocketAddr::from((self.address, self.port))
    }
    pub async fn to_state(&self) -> Result<AppState, AppError> {
        Ok(AppState {
            cookie_key: self.get_cookie_key().await?,
            db_pool: self.connect_db().await?,
            config: self.clone(),
        })
    }
}
