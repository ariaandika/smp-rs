use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::State, response::{IntoResponse, Redirect}, routing::{any, get}, Form, Router
};
use axum_extra::extract::CookieJar;
use rusqlite::OptionalExtension;
use serde::Deserialize;
use tera::Context;

use crate::{auth::{logout_cookie, Session, Users}, error::Error, page::TeraPage, Global};

const STOCK_PASSWD: &str = "$argon2id$v=19$m=19456,t=2,p=1$3lYyG6puInCkN/I/NXEQ9Q$CCIFuJ8fNDvSr0bPXYCoSCytqVvp0j7GTVmyLdNhrQQ";

pub fn routes() -> Router<Global> {
    Router::new()
        .route("/login", get(login_page).post(login))
        .route("/logout", any(logout))
        .nest("/admin", Router::new().route("/", get(admin_page)))
}

async fn admin_page(_: Session) -> Redirect {
    Redirect::to("/admin/events")
}

async fn login_page() -> TeraPage {
    TeraPage::render("login.html", Context::new())
}

async fn logout() -> (CookieJar, Redirect) {
    (CookieJar::new().add(logout_cookie()),Redirect::to("/login"))
}

async fn login(
    State(global): State<Global>,
    Form(login): Form<Login>,
) -> Result<LoginResponse, Error> {
    let user = tokio::task::spawn_blocking(move||{
        let name = login.name.clone();
        let db = global.conn.lock().unwrap();
        let mut stmt = db.prepare_cached("select * from users where name = $1")?;
        stmt.query_row(&[&name],Users::from_row).optional()
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

    Ok(LoginResponse::Ok((Session::new(user), Redirect::to("/admin"))))
}

#[derive(Debug, Deserialize)]
pub struct Login {
    name: String,
    #[allow(dead_code)]
    password: String,
}

enum LoginResponse {
    Ok((Session,Redirect)),
    Err(String),
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            LoginResponse::Ok(ok) => ok.into_response(),
            LoginResponse::Err(err) => {
                let mut ctx = Context::new();
                ctx.insert("error", &err);
                TeraPage::render("login.html", ctx).into_response()
            },
        }
    }
}

