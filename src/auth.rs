use std::{convert::Infallible, sync::LazyLock};

use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, IntoResponseParts, Redirect, Response, ResponseParts},
    RequestPartsExt,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

const COOKIE_KEY: &str = "token";
const COOKIE_RM: &str = "token=; Path=/";

static DECODE_KEY: LazyLock<DecodingKey> = LazyLock::new(|| DecodingKey::from_secret(b"Deez"));
static ENCODE_KEY: LazyLock<EncodingKey> = LazyLock::new(|| EncodingKey::from_secret(b"Deez"));

pub fn login(name: String) -> Session {
    Session { exp: 10000000000, name }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    exp: u64,
    name: String
}

impl Session {
    pub fn into_name(self) -> String {
        self.name
    }
}

impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = SessionRejection;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self,Self::Rejection> {
        let cookies = parts.extract::<CookieJar>().await.expect("infallible");
        let Some(cookie) = cookies.get(COOKIE_KEY) else {
            return Err(SessionRejection::NoSession);
        };

        let session = match decode::<Session>(cookie.value(), &DECODE_KEY, &Default::default()) {
            Ok(ok) => ok,
            Err(err) => {
                tracing::debug!("jwt error: {err}");
                return Err(SessionRejection::InvalidToken);
            }
        };

        Ok(session.claims)
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
            SessionRejection::InvalidToken => (
                [("Set-Cookie",COOKIE_RM)],
                Redirect::to("/login")
            ).into_response(),
        }
    }
}

pub struct SetSession(pub Session);

impl IntoResponseParts for SetSession {
    type Error = Infallible;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        let token = encode(&Default::default(), &self.0, &ENCODE_KEY).unwrap();
        CookieJar::new().add(Cookie::build((COOKIE_KEY,token)).path("/").build()).into_response_parts(res)
    }
}

