use axum::{
    http::HeaderValue,
    response::{IntoResponse, Response},
};
use axum::{routing::get, Router};
use hyper::{header, HeaderMap};

use crate::common::files::include_out_file;

pub const STYLESHEET_CONTENTS: &'static str = include_out_file!("/style.css");
pub const STYLESHEET_HASH: &'static str = include_out_file!("/style.hash");


async fn style_route() -> Response {
    let headers: HeaderMap = [
        (
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/css; charset=utf-8"),
        ),
        (
            header::CACHE_CONTROL,
            HeaderValue::from_static("max-age=604800"),
        ),
    ]
    .into_iter()
    .collect();
    (headers, STYLESHEET_CONTENTS).into_response()
}

pub fn router() -> Router {
    Router::new().route("/assets/:hash/style.css", get(style_route))
}
