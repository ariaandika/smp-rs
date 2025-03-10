use axum::Router;
use tower_http::services::ServeFile;



pub fn routes() -> Router {
    Router::new()
        .nest_service("/assets/output.css", ServeFile::new("assets/output.css"))
}

