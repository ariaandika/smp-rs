use rusqlite::Row;
use serde::{Deserialize, Serialize};

pub use session::{Session, logout_cookie};

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub name: String,
    #[serde(skip_serializing,default)]
    pub password: String,
}

impl Users {
    pub fn from_row(value: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            name: value.get("name")?,
            password: value.get("password")?,
        })
    }
}

mod session {
    use super::*;
    use crate::config;
    use axum::response::IntoResponseParts;
    use axum::{
        extract::FromRequestParts,
        http::request,
        response::{IntoResponse, Redirect, Response, ResponseParts},
    };
    use jsonwebtoken::{DecodingKey, EncodingKey};
    use std::sync::LazyLock;

    static DECODE_KEY: LazyLock<DecodingKey> =
        LazyLock::new(|| DecodingKey::from_secret(config::env("JWT_SECRET").as_bytes()));
    static ENCODE_KEY: LazyLock<EncodingKey> =
        LazyLock::new(|| EncodingKey::from_secret(config::env("JWT_SECRET").as_bytes()));

    const COOKIE_KEY: &str = "token";
    const SECURE: &str = if cfg!(debug_assertions) { "" } else { "; Secure" };

    /// set session cookie
    fn login_cookie(value: String) -> [(&'static str, String); 1] {
        [("Set-Cookie",format!("{COOKIE_KEY}={value}; Path=/; HttpOnly; Max-Age: 15552000{SECURE}"))]
    }

    /// removal session cookie
    pub fn logout_cookie() -> impl IntoResponseParts {
        [("Set-Cookie",format!("{COOKIE_KEY}=; Path=/; HttpOnly; Expires=Wed, 21 Oct 2015 07:28:00 GMT{SECURE}"))]
    }

    /// client session
    ///
    /// # `FromRequestParts`
    ///
    /// extract token from cookie and decode it to acquire session
    ///
    /// if session invalid or not found, redirect to /login
    ///
    /// # `IntoResponseParts`
    ///
    /// encode to a token then set a cookie
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Session {
        pub exp: u64,
        pub user: Users,
    }

    impl Session {
        /// create new session
        pub fn new(user: Users) -> Self {
            Self {
                exp: 10000000000,
                user,
            }
        }
    }

    impl<S> FromRequestParts<S> for Session
    where
        S: Send + Sync,
    {
        type Rejection = SessionRejection;

        async fn from_request_parts(parts: &mut request::Parts, _: &S) -> Result<Self, Self::Rejection> {
            let Some(token) = parts
                .headers
                .get("Cookie")
                .and_then(|e| e.to_str().ok())
                .and_then(|e| {
                    e.split("; ").find_map(|e| {
                        e.split_once("=")
                            .and_then(|(k, v)| (k == COOKIE_KEY).then_some(v))
                    })
                })
            else {
                return Err(SessionRejection::NoSession);
            };

            match jsonwebtoken::decode::<Session>(token, &DECODE_KEY, &Default::default()) {
                Ok(session) => Ok(session.claims),
                Err(_) => {
                    // tracing::debug!("jwt error: {err}");
                    Err(SessionRejection::InvalidToken)
                }
            }
        }
    }

    impl IntoResponseParts for Session {
        type Error = <[(&'static str, String); 1] as IntoResponseParts>::Error;

        fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
            login_cookie(jsonwebtoken::encode(&Default::default(), &self, &ENCODE_KEY).unwrap())
                .into_response_parts(res)
        }
    }

    /// [`Session`] error response
    pub enum SessionRejection {
        NoSession,
        InvalidToken,
    }

    impl IntoResponse for SessionRejection {
        fn into_response(self) -> Response {
            match self {
                SessionRejection::NoSession => Redirect::to("/login").into_response(),
                SessionRejection::InvalidToken => {
                    (logout_cookie(), Redirect::to("/login")).into_response()
                }
            }
        }
    }
}

