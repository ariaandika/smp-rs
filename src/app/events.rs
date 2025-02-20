use axum::{
    extract::{multipart, Multipart, State}, http::StatusCode, response::IntoResponse, routing::get, Json, Router,
};
use rand::{distr, Rng};
use serde::{Deserialize, Serialize};
use std::io;
use tera::Context;

use crate::{page::TeraPage, Global};

pub fn routes() -> Router<Global> {
    Router::new()
        .route("/events", get(list_page))
        .nest(
            "/admin",
            Router::new()
                .route("/events", get(list_page))
                .route("/events/add", get(add_page).post(add)),
        )
}

async fn list(State(global): State<Global>) -> Json<Vec<Events>> {
    Json(global.events.read().clone())
}

async fn list_page(state: State<Global>) -> TeraPage {
    let Json(events) = list(state).await;
    let mut c = Context::new();
    c.insert("events", &events);
    TeraPage::render("admin/events.html", c)
}

async fn add_page() -> TeraPage {
    TeraPage::render("admin/events.add.html", Context::new())
}

async fn add(
    State(global): State<Global>,
    mut form: Multipart,
) -> Result<(), AddEventError> {
    let mut title = None;
    let mut body = None;
    let mut image = None;

    while let Some(field) = form.next_field().await? {
        match field.name() {
            Some("title") => title = Some(field.text().await?),
            Some("body") => body = Some(field.text().await?),
            Some("image") => image = Some(field.bytes().await?),
            _ => {}
        }
    }

    let title = title.ok_or(AddEventError::Field("title"))?;
    let body = body.ok_or(AddEventError::Field("body"))?;
    let image = image.ok_or(AddEventError::Field("body"))?;

    let rng = rand::rng();
    let filename = rng.sample_iter(distr::Alphanumeric).take(5).map(char::from).collect::<String>();
    let filename = filename + ".png";
    tokio::fs::write(&format!("assets/local/{filename}"), image).await?;

    tokio::task::spawn_blocking(move||{
        {
            let crate::GlobalState { conn, events } = &*global;
            let conn = conn.lock().unwrap();
            {
                let mut stmt = conn.prepare("insert into events(title,body,image) values(?1,?2,?3)")?;
                stmt.execute((&title,&body,&filename))?;
            }
            {
                events.invalidate(&conn)?;
            }
        }
        Ok(())
    }).await?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Events {
    pub title: String,
    pub body: String,
    pub image: String,
}

#[derive(thiserror::Error, Debug)]
enum AddEventError {
    #[error(transparent)]
    Multipart(#[from] multipart::MultipartError),
    #[error("form field missing: {0}")]
    Field(&'static str),
    #[error("db error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("tokio error: {0}")]
    Tokio(#[from] tokio::task::JoinError),
}

impl IntoResponse for AddEventError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{self}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

