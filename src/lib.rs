use axum::Router;
use tower_http::services::ServeFile;

pub mod page;
pub mod error;

pub mod auth;
pub mod home;

pub fn routes() -> Router {
    Router::new()
        .route_service("/output.css", ServeFile::new("templates/output.css"))
        .merge(auth::routes())
        .merge(home::routes())
}


