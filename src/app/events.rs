use axum::{
    extract::{multipart, Multipart, Path, State}, http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post}, Json, Router,
};
use rand::{distr, Rng};
use serde::{Deserialize, Serialize};
use std::io;
use tera::Context;

use crate::{auth::Session, page::TeraPage, Global};

pub fn routes() -> Router<Global> {
    Router::new()
        .nest(
            "/admin/events",
            Router::new()
                .route("/", get(list_page))
                .route("/add", get(add_page).post(add))
                .route("/{id}/delete", post(rm)),
        )
}

pub async fn list(State(global): State<Global>) -> Json<Vec<Events>> {
    Json(global.events.read().clone())
}

async fn list_page(
    Session { user, .. }: Session,
    state: State<Global>
) -> TeraPage {
    let Json(events) = list(state).await;
    let mut c = Context::new();
    c.insert("events", &events);
    c.insert("name", &user.name);
    TeraPage::render("admin/events.html", c)
}

async fn add_page(Session { user, .. }: Session) -> TeraPage {
    let mut c = Context::new();
    c.insert("name", &user.name);
    TeraPage::render("admin/events.add.html", c)
}

async fn rm(
    Path(id): Path<u32>,
    _: Session,
    State(global): State<Global>,
) -> Result<Redirect, EventActionError> {
    tokio::task::spawn_blocking(move||{
        let crate::GlobalState { conn, events } = &*global;
        let conn = conn.lock().unwrap();
        let mut stmt = conn.prepare("delete from events where rowid = ?1")?;
        stmt.execute((id,))?;
        events.invalidate(&conn)?;
        Ok::<_, EventActionError>(())
    }).await??;

    Ok(Redirect::to("/admin/events"))
}

async fn add(
    State(global): State<Global>,
    mut form: Multipart,
) -> Result<Redirect, EventActionError> {
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

    let title = title.ok_or(EventActionError::Field("title"))?;
    let body = body.ok_or(EventActionError::Field("body"))?;
    let image = image.ok_or(EventActionError::Field("body"))?;

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
        Ok::<_, EventActionError>(())
    }).await??;

    Ok(Redirect::to("/admin/events"))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Events {
    pub rowid: usize,
    pub title: String,
    pub body: String,
    pub image: String,
}

#[derive(thiserror::Error, Debug)]
enum EventActionError {
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

impl IntoResponse for EventActionError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{self}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

