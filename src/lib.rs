use axum::{middleware, Router};
use std::sync::{LazyLock, RwLock};
use tera::Tera;
use tower_http::services::ServeDir;

pub mod page;
pub mod error;

pub mod auth;
pub mod home;

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
    let mut router = Router::new();

    if cfg!(debug_assertions) {
        router = router.layer(middleware::from_fn(page::debug::tera_reload));
    }

    router
        .nest_service("/assets", ServeDir::new("assets"))
        .merge(auth::routes())
        .merge(home::routes())
}


