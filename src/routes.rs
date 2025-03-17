use crate::{
    assets::{CAROUSEL, HX, CSS},
    config,
};
use axum::{routing::get, Router};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

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
        .with_state(Arc::new(state))

        .nest_service(CSS.serve_path, CSS)
        .nest_service(HX.serve_path, HX)
        .nest_service(CAROUSEL.serve_path, CAROUSEL)
}

