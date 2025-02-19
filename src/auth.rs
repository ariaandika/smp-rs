use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, IntoResponseParts, Redirect, Response, ResponseParts},
    RequestPartsExt,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::{convert::Infallible, sync::LazyLock};

pub use session::{Session, cookie, logout_cookie};

use crate::error::env;

/// `users` table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Users {
    pub name: String,
    #[serde(skip_serializing,default)]
    pub password: String,
}

mod session {
    use super::*;

    const COOKIE_KEY: &str = "token";

    static DECODE_KEY: LazyLock<DecodingKey> =
        LazyLock::new(|| DecodingKey::from_secret(env("JWT_SECRET").expect("checked").as_bytes()));
    static ENCODE_KEY: LazyLock<EncodingKey> =
        LazyLock::new(|| EncodingKey::from_secret(env("JWT_SECRET").expect("checked").as_bytes()));

    // following cookie functions is placed locally to maintain the same config
    // cookie builder type is private

    /// create set session cookie
    pub fn cookie(value: String) -> Cookie<'static> {
        let c = Cookie::build((COOKIE_KEY,value)).path("/");
        match cfg!(debug_assertions) {
            true => c,
            false => c.http_only(true).secure(true),
        }.build()
    }

    /// create removal session cookie
    pub fn logout_cookie<'a>() -> Cookie<'a> {
        let c = Cookie::build(COOKIE_KEY).removal().path("/");
        match cfg!(debug_assertions) {
            true => c,
            false => c.http_only(true).secure(true),
        }.build()
    }

    /// client session
    ///
    /// # `FromRequestParts`
    ///
    /// extract token from cookie and decode it to acquire session
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

    impl IntoResponseParts for Session {
        type Error = Infallible;

        fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
            let token = encode(&Default::default(), &self, &ENCODE_KEY).unwrap();
            CookieJar::new().add(cookie(token)).into_response_parts(res)
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
                SessionRejection::InvalidToken => (
                    CookieJar::new().add(logout_cookie()),
                    Redirect::to("/login"),
                )
                    .into_response(),
            }
        }
    }
}


