use std::sync::{Arc, Mutex};
use axum::{routing::get, Router};
use rusqlite::Connection;
use tower_http::services::ServeFile;
use crate::config;

mod auth;
mod home;
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
        .route("/login", get(auth::login_page).post(auth::login))
        .route("/session", get(auth::session))
        .route("/logout", get(auth::logout))
        .route("/library", get(library::page))
        .route("/library/books", get(library::books))
        .nest_service("/assets/output.css", ServeFile::new("assets/output.css"))
        .with_state(Arc::new(state))
}

