use axum::Router;

pub mod page;
pub mod error;

pub mod auth;
pub mod home;

pub fn routes() -> Router {
    Router::new()
        .merge(auth::routes())
        .merge(home::routes())
}


