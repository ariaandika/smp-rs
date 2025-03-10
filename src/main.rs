use anyhow::Context;
use tokio::net::TcpListener;

mod config;
mod routes;

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let tcp = std::net::TcpListener::bind(config::host())
        .with_context(|| format!("failed to bind {}", config::host()))?;
    tcp.set_nonblocking(true)?;
    let app = async {
        axum::serve(TcpListener::from_std(tcp)?, routes::routes())
            .await
            .map_err(Into::into)
    };
    tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .build()?
        .block_on(app)
}

