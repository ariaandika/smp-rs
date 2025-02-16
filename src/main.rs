use std::{env::var, io, net::TcpListener as StdTcp};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

const ADDR: &str = "localhost:3000";

fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    Error::var("JWT_SECRET")?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let tcp = StdTcp::bind(ADDR).map_err(Error::Tcp)?;
    tcp.set_nonblocking(true)?;

    let routes = smp_rs::routes();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let tcp = TcpListener::from_std(tcp).map_err(Error::Tcp)?;
            axum::serve(tcp, routes).await.map_err(Into::into)
        })
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("failed to bind tcp: {0}")]
    Tcp(io::Error),
    #[error("failed to get {0:?}: {1}")]
    Var(&'static str, std::env::VarError),
}

impl Error {
    fn var(name: &'static str) -> Result<(), Error> {
        match var(name) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Var(name, err)),
        }
    }
}

