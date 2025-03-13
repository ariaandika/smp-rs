use axum::{http::Uri, response::{Html, IntoResponse}, Form};
use serde::Deserialize;
use tour::Template;

#[derive(Template)]
#[template(path = "library/layout.html")]
pub struct Layout<'a, T: tour::Render> {
    body: T,
    path: &'a str
}


#[derive(Template)]
#[template(path = "library/index.html")]
pub struct Page {
}

#[derive(Template)]
#[template(path = "library/books.html")]
pub struct Books {
}

#[derive(Deserialize)]
pub struct AddBook {

}


pub async fn page(uri: Uri) -> impl IntoResponse {
    Html(Layout {
        body: Page { },
        path: uri.path(),
    }.render().unwrap())
}

pub async fn books(uri: Uri) -> impl IntoResponse {
    Html(Layout {
        body: Books { },
        path: uri.path(),
    }.render().unwrap())
}

pub async fn add_book(Form(form): Form<AddBook>) {
    
}

