use smp_rs::error::{env, SetupError};
use std::{net::TcpListener as StdTcp, process::exit};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

const ADDR: &str = "localhost:3000";

fn main() {
    if let Err(err) = app() {
        eprintln!("{err}");
        exit(1);
    }
}

fn app() -> Result<(), SetupError> {
    dotenvy::dotenv().ok();

    env("JWT_SECRET")?;
    env("DATABASE_URL")?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let tcp = StdTcp::bind(ADDR).map_err(SetupError::Tcp)?;
    tcp.set_nonblocking(true)?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let app = smp_rs::setup()?;
            let tcp = TcpListener::from_std(tcp).map_err(SetupError::Tcp)?;
            tracing::info!("listening: {}", tcp.local_addr().unwrap());
            axum::serve(tcp, app).await.map_err(Into::into)
        })
}

