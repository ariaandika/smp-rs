use axum::{http::Uri, response::{Html, IntoResponse}};
use tour::Template;


#[derive(Template)]
#[template(path = "presensi/presensi.html")]
pub struct Page {
    path: &'static str,
}

pub async fn page() -> impl IntoResponse {
    Html(Page { path: "/presensi" }.render_layout().unwrap())
}

