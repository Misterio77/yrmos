use axum::{response::Redirect, routing::get, Router, Server};
use clap::Parser;

use maud::Markup;
use yrmos::{
    common::{config::AppConfig, errors::AppError, style},
    layouts,
    routes::{login, logout, register, rides},
    schema::Session,
};

async fn home() -> Redirect {
    Redirect::to("/rides")
}
async fn fallback(session: Option<Session>) -> Markup {
    layouts::default(AppError::NotFound.as_html(), session.as_ref())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = AppConfig::parse();

    config.get_logger().init()?;
    let state = config.to_state().await?;
    let addr = config.get_addr();
    log::info!("Executando migrations em {}", config.database_url);
    sqlx::migrate!("db/migrations").run(&state.db_pool).await?;

    let app = Router::new()
        .fallback(fallback)
        .route("/", get(home))
        .merge(register::router(&state))
        .merge(login::router(&state))
        .merge(logout::router(&state))
        .merge(rides::router(&state))
        .with_state(state)
        .merge(style::router());
    log::info!("Ouvindo em {addr}");
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
