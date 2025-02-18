use axum::{routing::get, Router};
use rinja::Template;

use crate::{auth::Session, page::Page};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index))
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct Index {
    name: String,
}

pub async fn index(session: Session) -> Page<Index> {
    Page(Index {
        name: session.into_name(),
    })
}
