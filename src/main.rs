use axum::{routing::get, Router, Server};
use clap::Parser;

use maud::{html, Markup};
use yrmos::{layouts, AppConfig, AppError, StyleSheet};

static STYLE: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));

async fn style() -> StyleSheet {
    StyleSheet(STYLE.into())
}

async fn root() -> Result<Markup, AppError> {
    let body: Markup = html! {};
    // Ok(layouts::default(body))
    Err(AppError::NotAuthenticated)
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = AppConfig::parse();
    config.get_logger().init()?;

    let app = Router::new()
        .route("/", get(root))
        .route("/assets/style.css", get(style))
        .with_state(config.to_state().await?);

    let addr = config.get_addr();

    log::info!("Ouvindo em {addr}");
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
