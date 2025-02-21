use axum::{extract::State, routing::get, Json, Router};
use tera::Context;
use tower_http::services::ServeFile;

use crate::{app::events, page::TeraPage, Global};

pub fn routes() -> Router<Global> {
    Router::new()
        .route_service("/", ServeFile::new("templates/index.html"))
        .route("/events", get(home_events))
}

async fn home_events(state: State<Global>) -> TeraPage {
    let Json(events) = events::list(state).await;
    let mut c = Context::new();
    c.insert("events", &events);
    TeraPage::render("index.events.html", c)
}

