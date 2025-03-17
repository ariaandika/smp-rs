use std::sync::{Arc, Mutex};
use axum::{routing::get, Router};
use rusqlite::Connection;
use tower_http::services::ServeFile;
use crate::config;

mod auth;
mod home;
mod presensi;
mod library;

pub type Global = Arc<GlobalState>;

#[derive(Debug, Clone)]
pub struct GlobalState {
    conn: Arc<Mutex<Connection>>,
}

impl GlobalState {
    pub fn setup() -> anyhow::Result<Self> {
        Ok(Self {
            conn: Arc::new(Mutex::new(Connection::open(config::env("DATABASE_URL"))?)),
        })
    }
}

pub fn routes(state: GlobalState) -> Router {
    Router::new()
        .route("/", get(home::home))
        .merge(
            Router::new()
                .route("/login", get(auth::login_page).post(auth::login))
                .route("/session", get(auth::session))
                .route("/logout", get(auth::logout))
        )
        .nest(
            "/presensi",
            Router::new()
                .route("/", get(presensi::page))
        )
        .route("/library/books", get(library::books))
        .nest_service("/dist/output.css", ServeFile::new("dist/output.css"))
        .nest_service("/dist/hx.js", ServeFile::new("dist/hx.js"))
        .nest_service("/dist/carousel.js.", ServeFile::new("dist/carousel.js"))
        .with_state(Arc::new(state))
}

