use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::html;
use thiserror::Error;

use crate::layouts;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Você não está autenticado")]
    NotAuthenticated,
    #[error("Erro na base de dados")]
    Database(#[from] sqlx::Error),
    #[error("Erro ao calcular hash")]
    Hashing(#[from] argon2::Error),
    #[error("Erro ao iniciar logger")]
    Logging(#[from] log::SetLoggerError),
    #[error("Erro no servidor http")]
    Http(#[from] hyper::Error),
}

impl From<&AppError> for StatusCode {
    fn from(e: &AppError) -> StatusCode {
        match e {
            AppError::NotAuthenticated => StatusCode::UNAUTHORIZED,
            AppError::Database(e) => match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = StatusCode::from(&self);
        let pretty_status = format!(
            "{} ({})",
            status.as_str(),
            status.canonical_reason().unwrap_or_default()
        );
        let reason = format!("{:?}", &self);

        let main = html! {
            article {
                header {
                    h1 { (pretty_status) }
                }
                details open {
                    summary { "Erro detalhado:" }
                    code { (reason) }
                }
            }
        };
        let body = layouts::default(main);
        (status, body).into_response()
    }
}
