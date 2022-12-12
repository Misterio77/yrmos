use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use maud::html;
use thiserror::Error;

use crate::layouts;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Você não está autenticado, faça login")]
    NotAuthenticated,
    #[error("Sua sessão expirou, faça login novamente")]
    InvalidSession,
    #[error("Credenciais inválidas, tente novamente")]
    InvalidCredentials,
    #[error("Página ou recuso não encontrado")]
    NotFound,
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
            AppError::InvalidSession => StatusCode::UNAUTHORIZED,
            AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AppError::NotFound => StatusCode::NOT_FOUND,
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
        match self {
            AppError::NotAuthenticated | AppError::InvalidSession => {
                (Redirect::to("/login")).into_response()
            }
            _ => {
                let status = StatusCode::from(&self);
                let main = html! {
                    article {
                        header {
                            hgroup {
                                h1 {
                                    (status.as_str())
                                    " "
                                    (status.canonical_reason().unwrap_or_default())
                                }
                                h2 {
                                    (self.to_string())
                                }
                            }
                        }
                        p {
                            "Tente "
                            a onclick="history.go(-1)" { "voltar" }
                            " ou "
                            a href="/" { "ir para a home" }
                            "."
                        }
                        hr;
                        details {
                            summary { "Detalhes do erro:" }
                            pre { code { (format!("{:?}", self)) } }
                        }
                    }
                };
                let body = layouts::root(main, None, false);
                (status, body).into_response()
            }
        }
    }
}
