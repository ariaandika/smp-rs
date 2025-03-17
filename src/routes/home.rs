use axum::response::{Html, IntoResponse};
use tour::Template;



#[derive(Template)]
#[template(path = "index.html")]
struct Home;

pub async fn home() -> impl IntoResponse {
    Html(Home.render_layout().unwrap())
}

