use std::{io, net::TcpListener as StdTcp};

use axum::{response::Html, routing::get, Router};
use rinja::Template;
use tokio::net::TcpListener;


#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("failed to bind tcp: {0}")]
    Tcp(io::Error),
}

const ADDR: &str = "localhost:3000";

fn main() -> Result<(), Error> {
    let tcp = StdTcp::bind(ADDR).map_err(Error::Tcp)?;
    tcp.set_nonblocking(true)?;

    let routes = Router::new()
        .route("/", get(hello));

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let tcp = TcpListener::from_std(tcp).map_err(Error::Tcp)?;
            Ok(axum::serve(tcp, routes).await?)
        })
}

#[derive(Template)]
#[template(
    source = "<div>Nice</div>",
    ext = "html"
)]
struct App {

}

async fn hello() -> Html<String> {
    Html(App { }.render().unwrap())
}
