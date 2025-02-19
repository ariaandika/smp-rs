use std::{future::{ready, Ready}, sync::Arc};

use axum::{extract::Request, handler::Handler, http::StatusCode, response::{Html, IntoResponse, Response}};
use rinja::Template;
use tera::Context;

use crate::TERA;

//
// TeraPage
//

pub struct TeraPage {
    name: &'static str,
    context: Context,
}

impl TeraPage {
    pub fn render(name: &'static str, context: Context) -> TeraPage {
        Self { name, context }
    }
}

impl IntoResponse for TeraPage {
    fn into_response(self) -> Response {
        match TERA.read().unwrap().render(self.name, &self.context) {
            Ok(ok) => Html(ok).into_response(),
            Err(err) => {
                tracing::error!("render error: {err}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
        }
    }
}

//
// Page
//

pub struct Page<T>(pub T);

impl<T> IntoResponse for Page<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(ok) => Html(ok).into_response(),
            Err(err) => {
                tracing::error!("render error: {err}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html("Internal Server Error"),
                ).into_response()
            },
        }
    }
}

//
// PageHandler
//

pub struct PageHandler<T>(Arc<T>);

impl<T> PageHandler<T> {
    pub fn new(templ: T) -> PageHandler<T> {
        Self(Arc::new(templ))
    }
}

impl<T,S> Handler<((),),S> for PageHandler<T>
where
    T: Template + Send + Sync + 'static,
{
    type Future = Ready<Response>;

    fn call(self, _: Request, _: S) -> Self::Future {
        ready(match self.0.render() {
            Ok(ok) => Html(ok).into_response(),
            Err(err) => {
                tracing::error!("render error: {err}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html("Internal Server Error"),
                ).into_response()
            },
        })
    }
}

impl<T> Clone for PageHandler<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}


#[cfg(debug_assertions)]
pub mod debug {
    use std::future::Future;
    use axum::{extract::Request, middleware::Next, response::Response};

    use crate::TERA;

    pub fn tera_reload(req: Request, next: Next) -> impl Future<Output = Response> {
        if let Err(err) = TERA.write().unwrap().full_reload() {
            tracing::error!("template error: {err}");
        }
        next.run(req)
    }
}

