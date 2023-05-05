use axum::response::{IntoResponse, Response};
use axum::{routing::get, Router};

pub static VERSION: &str = env!("CARGO_PKG_VERSION");

async fn version_route() -> Response {
    VERSION.into_response()
}

pub fn router() -> Router {
    Router::new().route("/version", get(version_route))
}
