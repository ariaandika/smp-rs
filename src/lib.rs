use axum::{middleware, Router};
use std::sync::{LazyLock, RwLock};
use tera::Tera;
use tower_http::services::ServeDir;

pub mod page;
pub mod error;

pub mod auth;
pub mod handlers;

pub static TERA: LazyLock<RwLock<Tera>> = LazyLock::new(||{
    match Tera::new("templates/**/*.html") {
        Ok(ok) => RwLock::new(ok),
        Err(err) => {
            eprintln!("Parsing error: {err}");
            ::std::process::exit(1);
        }
    }
});

pub fn routes() -> Router {
    let router = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .merge(handlers::auth::routes())
        .merge(handlers::home::routes());

    if cfg!(debug_assertions) {
        router.layer(middleware::from_fn(page::debug::tera_reload))
    } else {
        router
    }
}


