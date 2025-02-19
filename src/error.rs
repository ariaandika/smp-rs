use std::io;

use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("tokio error: {0}")]
    Tokio(#[from] tokio::task::JoinError),
}

impl Error {
    fn log(&self) {
        let err: &dyn std::fmt::Display = match self {
            Error::Db(err) => err as _,
            Error::Tokio(err) => err as _,
        };

        tracing::error!("{err}");
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        self.log();
        match self {
            Error::Db(_) | Error::Tokio(_)
                => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetupError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("failed to bind tcp: {0}")]
    Tcp(io::Error),
    #[error("failed to get {0:?}: {1}")]
    Var(&'static str, std::env::VarError),
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
}

pub fn env(name: &'static str) -> Result<String, SetupError> {
    match std::env::var(name) {
        Ok(val) => Ok(val),
        Err(err) => Err(SetupError::Var(name, err)),
    }
}

