use super::Global;
use crate::{
    auth::{self, Session, Users},
    error::Error,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::State, response::{Html, IntoResponse, Redirect}, Form, Json};
use rinja::Template;
use rusqlite::OptionalExtension;
use serde::Deserialize;

const STOCK_PASSWD: &str = "$argon2id$v=19$m=19456,t=2,p=1$3lYyG6puInCkN/I/NXEQ9Q$CCIFuJ8fNDvSr0bPXYCoSCytqVvp0j7GTVmyLdNhrQQ";

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginPage {
    error: Option<String>,
}

pub async fn login_page() -> impl IntoResponse {
    Html(LoginPage { error: None }.render().unwrap())
}

#[derive(Debug, Deserialize)]
pub struct Login {
    name: String,
    #[allow(dead_code)]
    password: String,
}

pub async fn login(
    State(global): State<Global>,
    Form(login): Form<Login>,
) -> impl IntoResponse {
    let user = tokio::task::spawn_blocking(move||{
        let name = login.name.clone();
        let db = global.conn.lock().unwrap();
        let mut stmt = db.prepare_cached("select * from users where name = $1")?;
        stmt.query_row([&name],Users::from_row).optional()
    }).await?? ;

    let passwd = login.password.clone();
    let hashed = user.as_ref().map(|e|e.password.clone());

    let result = tokio::task::spawn_blocking(move || {
        let hashed = PasswordHash::new(hashed.as_deref().unwrap_or(STOCK_PASSWD));
        if let Err(err) = hashed {
            tracing::debug!("argon2 error: {err}");
        }
        Argon2::default().verify_password(passwd.as_bytes(), &hashed?)
    })
    .await?;

    let Some(user) = user else {
        return Ok(LoginResponse::Err("username atau password salah".into()));
    };

    if let Err(err) = result {
        tracing::debug!("login error: {err}");
        return Ok(LoginResponse::Err("username atau password salah".into()));
    }

    Ok::<_, Error>(LoginResponse::Ok((Session::new(user), Redirect::to("/admin"))))
}

pub enum LoginResponse {
    Ok((Session,Redirect)),
    Err(String),
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            LoginResponse::Ok(ok) => ok.into_response(),
            LoginResponse::Err(err) => {
                Html(LoginPage { error: Some(err) }.render().unwrap()).into_response()
            },
        }
    }
}

pub async fn session(user: Session) -> impl IntoResponse {
    Json(user)
}

pub async fn logout() -> impl IntoResponse {
    (auth::logout_cookie(),())
}

