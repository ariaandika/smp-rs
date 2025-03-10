use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("db error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("tokio error: {0}")]
    Tokio(#[from] tokio::task::JoinError),
}

impl Error {
    fn log(&self) {
        match self {
            Error::Db(err) => tracing::error!("{err}"),
            Error::Tokio(err) => tracing::error!("{err}"),
        }
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
