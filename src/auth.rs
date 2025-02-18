use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, IntoResponseParts, Redirect, Response, ResponseParts},
    routing::{any, post},
    Form, RequestPartsExt, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, future::ready, sync::LazyLock};

pub use session::{Session, SetSession};

const COOKIE_KEY: &str = "token";
const COOKIE_RM: &str = "token=; Path=/";

static DECODE_KEY: LazyLock<DecodingKey> = LazyLock::new(|| DecodingKey::from_secret(b"Deez"));
static ENCODE_KEY: LazyLock<EncodingKey> = LazyLock::new(|| EncodingKey::from_secret(b"Deez"));

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route(
            "/logout",
            any(|| {
                ready((
                    CookieJar::new().add(Cookie::build("token").removal().path("/").build()),
                    Redirect::to("/login"),
                ))
            }),
        )
}

#[derive(Debug, Deserialize)]
pub struct Login {
    name: String,
    #[allow(dead_code)]
    password: String,
}

pub async fn login(Form(login): Form<Login>) -> (SetSession, Redirect) {
    (SetSession(Session::new(login.name)),Redirect::to("/"))
}

mod session {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Session {
        exp: u64,
        name: String,
    }

    impl Session {
        pub fn new(name: String) -> Self {
            Self {
                exp: 10000000000,
                name,
            }
        }

        pub fn into_name(self) -> String {
            self.name
        }
    }

    impl<S> FromRequestParts<S> for Session
    where
        S: Send + Sync,
    {
        type Rejection = SessionRejection;

        async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
            let cookies = parts.extract::<CookieJar>().await.expect("infallible");
            let Some(cookie) = cookies.get(COOKIE_KEY) else {
                return Err(SessionRejection::NoSession);
            };

            match decode::<Session>(cookie.value(), &DECODE_KEY, &Default::default()) {
                Ok(session) => Ok(session.claims),
                Err(err) => {
                    tracing::debug!("jwt error: {err}");
                    Err(SessionRejection::InvalidToken)
                }
            }
        }
    }

    pub enum SessionRejection {
        NoSession,
        InvalidToken,
    }

    impl IntoResponse for SessionRejection {
        fn into_response(self) -> Response {
            match self {
                SessionRejection::NoSession => Redirect::to("/login").into_response(),
                SessionRejection::InvalidToken => {
                    ([("Set-Cookie", COOKIE_RM)], Redirect::to("/login")).into_response()
                }
            }
        }
    }

    pub struct SetSession(pub Session);

    impl IntoResponseParts for SetSession {
        type Error = Infallible;

        fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
            let token = encode(&Default::default(), &self.0, &ENCODE_KEY).unwrap();
            CookieJar::new()
                .add(Cookie::build((COOKIE_KEY, token)).path("/").build())
                .into_response_parts(res)
        }
    }
}


