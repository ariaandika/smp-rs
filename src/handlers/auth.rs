use axum::{
    response::Redirect,
    routing::{any, post},
    Form, Router,
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::auth::{Session, logout_cookie};

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", any(logout))
}

async fn login(Form(login): Form<Login>) -> (Session, Redirect) {
    (Session::new(login.name),Redirect::to("/"))
}

async fn logout() -> (CookieJar, Redirect) {
    (CookieJar::new().add(logout_cookie()),Redirect::to("/login"))
}

#[derive(Debug, Deserialize)]
pub struct Login {
    name: String,
    #[allow(dead_code)]
    password: String,
}

