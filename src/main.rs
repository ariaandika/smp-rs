use anyhow::Context;
use tokio::net::TcpListener;

mod config;
mod helpers;

mod assets;
mod auth;
mod models;
mod routes;
mod error;

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    config::env_assert("DATABASE_URL")?;

    let state = routes::GlobalState::setup()?;

    let tcp = std::net::TcpListener::bind(config::host())
        .with_context(|| format!("failed to bind {}", config::host()))?;
    tcp.set_nonblocking(true)?;

    #[cfg(debug_assertions)]
    assets::tw_build();

    tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .build()?
        .block_on(async {
            axum::serve(TcpListener::from_std(tcp)?, routes::routes(state))
                .await
                .map_err(Into::into)
        })
}

