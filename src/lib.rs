use axum::{middleware, Router};
use cache::events::EventCache;
use error::{env, SetupError};
use rusqlite::Connection;
use std::sync::{Arc, LazyLock, Mutex, RwLock};
use tera::Tera;
use tower_http::services::ServeDir;

pub mod page;
pub mod cache;
pub mod error;

pub mod auth;
pub mod app;

pub static TERA: LazyLock<RwLock<Tera>> = LazyLock::new(||{
    match Tera::new("templates/**/*.html") {
        Ok(ok) => RwLock::new(ok),
        Err(err) => {
            eprintln!("Parsing error: {err}");
            ::std::process::exit(1);
        }
    }
});

pub type Global = Arc<GlobalState>;

#[derive(Debug, Clone)]
pub struct GlobalState {
    conn: Arc<Mutex<Connection>>,
    events: EventCache,
}

pub fn setup() -> Result<Router, SetupError> {
    let mut conn = Connection::open(&env("DATABASE_URL")?)?;

    let events = cache::events::EventCache::setup(&mut conn)?;

    let state = GlobalState {
        conn: Arc::new(Mutex::new(conn)),
        events,
    };

    let router = Router::new()
            .nest_service("/assets", ServeDir::new("assets"))
            .merge(app::auth::routes())
            .merge(app::home::routes())
            .merge(app::events::routes())
            .merge(if cfg!(debug_assertions) {
                Router::new().layer(middleware::from_fn(page::debug::tera_reload))
            } else {
                Router::new()
            })
            .with_state(Arc::new(state));

    Ok(router)
}


