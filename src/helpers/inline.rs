use axum::{
    body::Bytes,
    extract::Request,
    http::{
        header::{CACHE_CONTROL, CONTENT_TYPE, ETAG, IF_NONE_MATCH, LAST_MODIFIED},
        HeaderValue, StatusCode,
    },
    response::{IntoResponse, Response},
};
use std::{
    convert::Infallible,
    fs,
    future::{ready, Ready},
};
use tower_service::Service;

#[derive(Clone)]
pub struct InlineAssets {
    pub path: &'static str,
    pub serve_path: &'static str,
    pub content: &'static [u8],
    pub tag: &'static str,
    pub last_modified: &'static str,
    pub mime: &'static str,
    pub dev_reload: bool,
}

impl Service<Request> for InlineAssets {
    type Response = Response;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response,Self::Error>>;

    fn poll_ready(&mut self, _: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        ready(Ok(match () {
            #[cfg(debug_assertions)]
            _ if self.dev_reload => IntoResponse::into_response((
                [
                    (CONTENT_TYPE, HeaderValue::from_static(self.mime)),
                    (CACHE_CONTROL, HeaderValue::from_static("no-cache")),
                ],
                {
                    println!("CSS no cache");
                    fs::read_to_string(self.path).unwrap()
                },
            )),

            _ if check_etag(&req, self.tag) => (StatusCode::NOT_MODIFIED, ()).into_response(),

            // cache miss
            _ => IntoResponse::into_response((
                [
                    (CACHE_CONTROL, HeaderValue::from_static("max-age=3600")), // one hour
                    (ETAG, HeaderValue::from_static(self.tag)),
                    (LAST_MODIFIED, HeaderValue::from_static(self.last_modified)),
                    (CONTENT_TYPE, HeaderValue::from_static(self.mime)),
                ],
                Bytes::from_static(self.content),
            )),
        }))
    }
}

fn check_etag(req: &Request, tag: &str) -> bool {
    req.headers()
        .get(IF_NONE_MATCH)
        .map(|etag| etag.as_bytes() == tag.as_bytes())
        .unwrap_or(false)
}


