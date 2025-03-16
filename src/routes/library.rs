use axum::{http::Uri, response::{Html, IntoResponse}, Form};
use serde::Deserialize;
use tour::Template;

#[derive(Template)]
#[template(path = "library/books.html")]
pub struct Books {
}

#[derive(Deserialize)]
pub struct AddBook {

}


pub async fn books(uri: Uri) -> impl IntoResponse {
    // Html(Layout {
    //     body: Books { },
    //     path: uri.path(),
    // }.render().unwrap())
    Html("")
}

pub async fn add_book(Form(form): Form<AddBook>) {
    
}

