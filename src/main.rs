use std::{io, process};
use vice::router::{get, Router};

fn main() {
    if let Err(err) = app() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn app() -> io::Result<()> {
    dotenvy::dotenv().ok();
    Router::new()
        .route("/", get(index))
        .listen("0.0.0.0:3000")
}

async fn index() -> &'static str {
    "Nice"
}

