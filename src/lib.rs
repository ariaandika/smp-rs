use axum::{response::Html, Router};
use rinja::Template;

pub mod auth;

pub fn routes() -> Router {
    Router::new()
        .route("/", axum::routing::get(index))
        .merge(auth::routes())
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    name: String,
}

async fn index(session: auth::Session) -> Html<String> {
    Html(
        Index {
            name: session.into_name(),
        }
        .render()
        .unwrap_or_else(|e| e.to_string()),
    )
}


