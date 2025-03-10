use std::{io, process};

use axum::{response::IntoResponse, routing::get, Router};
use rinja::Template;
use tokio::net::TcpListener;
use tower_http::services::ServeFile;

fn main() {
    dotenvy::dotenv().ok();
    if let Err(err) = app() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn app() -> io::Result<()> {
    let tcp = std::net::TcpListener::bind("0.0.0.0:3000")?;
    tcp.set_nonblocking(true)?;

    let routes = Router::new()
        .nest_service("/assets/output.css", ServeFile::new("assets/output.css"))
        .route("/", get(page));

    tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .build()?
        .block_on(async {
            let tcp = TcpListener::from_std(tcp)?;
            axum::serve(tcp, routes).await
        })
}

async fn page() -> impl IntoResponse {
    ([("content-type",Page::MIME_TYPE)],Page { }.render().unwrap())
}

#[derive(Template)]
#[template(path = "index.html")]
struct Page {

}

