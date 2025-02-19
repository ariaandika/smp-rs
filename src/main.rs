use axum::Extension;
use smp_rs::error::{env, SetupError};
use sqlx::postgres::PgPoolOptions;
use std::{net::TcpListener as StdTcp, process::exit, time::Duration};
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
            let db = PgPoolOptions::new()
                .acquire_timeout(Duration::from_secs(5))
                .connect_lazy(&env("DATABASE_URL")?)?;

            let routes = smp_rs::routes()
                .layer(Extension(db));

            tracing::info!("listening: {}", tcp.local_addr().unwrap());

            let tcp = TcpListener::from_std(tcp).map_err(SetupError::Tcp)?;
            axum::serve(tcp, routes).await.map_err(Into::into)
        })
}

