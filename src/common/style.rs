use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use hyper::{
    header::{self, HeaderValue},
    HeaderMap,
};

pub struct StyleSheet(pub String);

impl IntoResponse for StyleSheet {
    fn into_response(self) -> Response {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/css; charset=utf-8"),
        );
        (headers, self.0).into_response()
    }
}

static STYLE: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));

async fn style_route() -> StyleSheet {
    StyleSheet(STYLE.into())
}

pub fn router() -> Router {
    Router::new().route("/assets/style.css", get(style_route))
}
