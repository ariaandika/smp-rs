use std::{io, process};
use tokio::net::TcpListener;

mod routes;

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
    let app = async { axum::serve(TcpListener::from_std(tcp)?, routes::routes()).await };
    tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .build()?
        .block_on(app)
}

