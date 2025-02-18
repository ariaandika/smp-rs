use axum::Router;
use tower_http::services::ServeFile;

pub fn routes() -> Router {
    Router::new()
        .route_service("/", ServeFile::new("templates/home.html"))
}


