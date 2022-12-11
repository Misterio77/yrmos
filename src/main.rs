use anyhow::Result;
use axum::{Router, Server};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let config = yrmos::AppConfig::parse();
    config.get_logger().init()?;

    let router = Router::new();
    let app = router.with_state(config.to_state().await?);

    Server::bind(&config.get_addr())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
