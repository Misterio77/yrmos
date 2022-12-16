use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use maud::{html, Markup};
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
    #[error("Você não pode fazer essa ação")]
    NotAllowed,
    #[error("Erro ao migrar base de dados")]
    DatabaseMigration(#[from] sqlx::migrate::MigrateError),
    #[error("Erro na base de dados")]
    Database(#[from] sqlx::Error),
    #[error("Erro ao calcular hash")]
    Hashing(#[from] argon2::Error),
    #[error("Erro ao iniciar logger")]
    Logging(#[from] log::SetLoggerError),
    #[error("Erro no servidor http")]
    Http(#[from] hyper::Error),
}

impl AppError {
    pub fn redirect(&self, uri: &str) -> Redirect {
        Redirect::to(&format!("{uri}?error={self}"))
    }
    pub fn as_html(&self) -> Markup {
        let status = StatusCode::from(self);
        html! {
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
        }
    }
}

impl From<&AppError> for StatusCode {
    fn from(e: &AppError) -> StatusCode {
        match e {
            AppError::NotAuthenticated => StatusCode::UNAUTHORIZED,
            AppError::InvalidSession => StatusCode::UNAUTHORIZED,
            AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::NotAllowed => StatusCode::FORBIDDEN,
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
                (Redirect::to(&format!("/login?error={self}"))).into_response()
            }
            _ => {
                let status = StatusCode::from(&self);
                let body = layouts::root(self.as_html(), None, false);
                (status, body).into_response()
            }
        }
    }
}
