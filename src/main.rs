use std::{io, net::TcpListener as StdTcp};

use auth::{Session, SetSession};
use axum::{response::{Html, Redirect}, routing::get, Form, Router};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use rinja::Template;
use serde::Deserialize;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

mod auth;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("failed to bind tcp: {0}")]
    Tcp(io::Error),
}

const ADDR: &str = "localhost:3000";

fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let tcp = StdTcp::bind(ADDR).map_err(Error::Tcp)?;
    tcp.set_nonblocking(true)?;

    let routes = Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
        .route("/logout", get(logout).post(logout));

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let tcp = TcpListener::from_std(tcp).map_err(Error::Tcp)?;
            Ok(axum::serve(tcp, routes).await?)
        })
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    name: String,
}

async fn index(session: Session) -> Html<String> {
    Html(Index { name: session.into_name() }.render().unwrap_or_else(|e|e.to_string()))
}


#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

async fn login_page() -> Html<String> {
    Html(LoginPage.render().unwrap_or_else(|e|e.to_string()))
}

#[derive(Debug, Deserialize)]
struct Login {
    name: String,
    #[allow(dead_code)]
    password: String,
}

async fn login(Form(login): Form<Login>) -> (SetSession, Redirect) {
    let session = auth::login(login.name);
    (SetSession(session),Redirect::to("/"))
}

async fn logout() -> (CookieJar, Redirect) {
    (CookieJar::new().add(Cookie::build("token").removal().path("/").build()), Redirect::to("/login"))
}

