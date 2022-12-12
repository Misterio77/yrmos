use axum::{routing::get, Router, Server};
use clap::Parser;

use maud::{html, Markup};
use yrmos::{
    common::{config::AppConfig, errors::AppError, style},
    layouts,
    routes::{login, register},
    schema::Session,
};

async fn home(session: Option<Session>) -> Markup {
    layouts::default(html! {}, session.as_ref())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = AppConfig::parse();
    config.get_logger().init()?;
    let state = config.to_state().await?;

    let app = Router::new()
        .fallback(|| async { AppError::NotFound })
        .route("/", get(home))
        .merge(register::router(&state))
        .merge(login::router(&state))
        .with_state(state)
        .merge(style::router());

    let addr = config.get_addr();

    log::info!("Ouvindo em {addr}");
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
