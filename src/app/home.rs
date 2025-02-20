use axum::Router;
use tower_http::services::ServeFile;

use crate::Global;

pub fn routes() -> Router<Global> {
    Router::new()
        .route_service("/", ServeFile::new("templates/index.html"))
}


